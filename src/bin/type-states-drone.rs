#![doc = ::embed_doc_image::embed_image!("drone-states-diagram", "docs/images/drone-type-states-diagram.png")]
//! 这段例程代码，通过给“操控【无人机】飞行”建立【程序模型】，来演示【类型·状态】设计模式的编码套路。
//!
//! 在此例程中，被涉及到的子技术知识点包括：
//! 1. 零抽象成本·状态字段（见`Flying<S: Motionless>.destination_state`）
//! 2. 按【智能指针】存储的多个状态共有字段值（见`Drone<S: State>.coordinate`）
//! 3. 状态类型·分组（见`group_by_trait!()`宏）
//! 4. 【状态组】独有成员方法（见`Drone<S: Midair>::take_picture(&self)`和`Drone<Flying<S: Motionless>>::inner_fly(mut self, state, step)`）
//! 5. 密封【状态类型】以禁止下游代码扩展额外状态（见`seal_by_trait!()`宏）
//! 6. 编译时多态的【状态过渡】（见`Drone<Flying<Idle>>::fly(mut self, step)`和`Drone<Flying<Hovering>>::fly(mut self, step)`）
//! 7. 【状态】独有成员方法（见`Drone<Idle>::take_off(self)`）
//! 8. 【状态】独有数据缓存字段（见`Flying<S: Motionless>.origin`）
//!
//! 【无人机】飞行过程与状态结点包括：
//!
//! ![无人机·飞行状态图][drone-states-diagram]
//!
//! 【无人机】总共在三个状态之间切换：
//! 1. 待命`Idle` —— 无人机·在地面上
//! 2. 飞行`Flying` —— 无人机·空中飞行
//! 3. 悬浮`Hovering` —— 无人机·原地悬浮于空中
//!
//! 接着，这三个状态又按两个维度分成了两组：
//! 1. “静止”状态组`Motionless`，包括`Idle`和`Hovering`
//!     * `Flying`状态的紧下一个状态必须是“静止组”内的状态。
//! 2. “空中”状态组`Midair`，包括`Flying`和`Hovering`
//!     * 空中的无人机有一个额外的功能就是“拍照”。而停在地面上不能拍照。
//!
//! 【无人机】三个状态各有独特的行为：
//! 1. `Idle`有`take_off()`起飞·行为，从而将`Idle`状态过渡为`Flying`
//! 2. `Hovering`有`move_to()`前往·与`land()`着落·两个行为，从而将`Hovering`状态过渡为`Flying`
//! 3. `Flying`有`fly()`飞行·行为。该行为
//!     1. 既是【异步】的：
//!         * 用跨线程【迭代器】模拟【无人机】（缓慢）飞行过程。
//!     2. 还是【多态】的：
//!         1. 若紧前状态是`Idle`，那么当前状态过渡的目标就一定是`Hovering`。即，`Idle -> Flying -> Hovering`
//!         2. 若紧前状态是`Hovering`，那么当前状态过渡的目标既有可能是`Idle`，还可能还是`Hovering`。这取决于之前`Hovering`是如何过渡到`Flying`的。
//!
//!         `fly()`行为的输出状态是不确定的，得看它的紧上一个状态是什么！
//!
mod drone_model {
    /// 收拢了几段值得复用宏的宏工具箱
    #[macro_use]
    mod macro_utils {
        /// 宏功能：限定 + 密封【状态·类型】
        /// 意图：使公开的【业务功能】接口拒绝接收“下游”代码自定义的任
        /// 何【状态·类型】。
        macro_rules! seal_by_trait {
            (
                @private $subtrait: ident,
                $supertrait_mod: ident,
                [$($state1: ident$(<$state_generic1: ident>)?),*],
                [$($state2: ident<$state_generic2: ident: $where: ident>),*]
            ) => {
                /// （私有的）密封`trait`
                mod $supertrait_mod {
                    pub trait Sealed {}
                }
                /// （公开的）【状态·类型】`trait`对外不可实现，
                /// 因为它继承了（私有的）密封`trait`。
                pub trait $subtrait: $supertrait_mod::Sealed {}
                /// 给【状态·类型】限定条件
                $(
                    impl $supertrait_mod::Sealed for $state1$(<$state_generic1>)? {}
                    impl $subtrait for $state1$(<$state_generic1>)? {}
                )+
                $(
                    impl<$state_generic2: $where> $supertrait_mod::Sealed for $state2<$state_generic2> {}
                    impl<$state_generic2: $where> $subtrait for $state2<$state_generic2> {}
                )*
            };
            // 这是【宏】入口，提供了默认的【密封`trait`】名。
            (
                $subtrait: ident,
                [$($state1: ident$(<$state_generic1: ident>)?),*],
                [$($state2: ident<$state_generic2: ident: $where: ident>),*]
            ) => {
                seal_by_trait!(
                    @private $subtrait,
                    sealed,
                    [$($state1$(<$state_generic1>)?),*],
                    [$($state2<$state_generic2: $where>),*]
                );
            };
            (
                $subtrait: ident,
                [$($state1: ident$(<$state_generic1: ident>)?),*]
            ) => {
                seal_by_trait!(
                    @private $subtrait,
                    [$($state1$(<$state_generic1>)?),*],
                    []
                );
            };
            (
                $subtrait: ident,
                [$($state2: ident<$state_generic2: ident: $where: ident>),*]
            ) => {
                seal_by_trait!(
                    @private $subtrait,
                    [],
                    [$($state2<$state_generic2: $where>),*]
                );
            };
        }
        /// 宏功能：分组【状态·类型】
        /// 意图：使每组【状态·类型】都有（组内）独有的【成员方法】。
        macro_rules! group_by_trait {
            (
                $group: ident $(: $supertrait: ident)?,
                [$($state1: ident$(<$state_generic1: ident>)?),*],
                [$($state2: ident<$state_generic2: ident: $where: ident>),*]
            ) => {
                /// 定义·分组`marker trait`
                pub trait $group $(:$supertrait)? {}
                /// 通过标记【状态·类型】，将【状态·类型】分组
                $(
                    impl $group for $state1$(<$state_generic1>)? {}
                )*
                $(
                    impl<$state_generic2: $where> $group for $state2<$state_generic2> {}
                )*
            };
            (
                $group: ident $(: $supertrait: ident)?,
                [$($state1: ident$(<$state_generic1: ident>)?),*]
            ) => {
                group_by_trait!(
                    $group $(: $supertrait)?,
                    [$($state1$(<$state_generic1>)?),*],
                    []
                );
            };
            (
                $group: ident $(: $supertrait: ident)?,
                [$($state2: ident<$state_generic2: ident: $where: ident>),*]
            ) => {
                group_by_trait!(
                    $group $(: $supertrait)?,
                    [],
                    [$($state2<$state_generic2: $where>),*]
                );
            }
        }
        /// 宏功能：若前一段占用互斥锁的程序运行崩溃了，当前程序依旧获取
        /// 被保护数据的读写权限，而不是随着前一段程序一起崩溃掉。但，此
        /// 决策是有读到脏数据风险的。
        macro_rules! get_mutex_lock {
            ($mutex: expr, $closure: expr) => {
                match $mutex.lock() {
                    Ok(mut value) => $closure(&mut value),
                    Err(mut err) => $closure(err.get_mut())
                }
            };
        }
    }
    /// 定义了【无人机】的三个状态。并对这些状态进行
    /// 1. 限定 + 密封 —— 禁止“下游”代码扩展
    /// 2. 分组 —— 静止状态组·和·空中状态组
    mod drone_states {
        use ::async_std::task::JoinHandle;
        use ::derive_builder::Builder;
        use ::std::marker::PhantomData;
        use super::Coordinate;
        // -------------------------
        // 状态·类型 — 描述·无人机·工作状态
        // -------------------------
        /// 无人机·在地面上
        pub struct Idle;
        /// 无人机·原地悬浮于空中
        pub struct Hovering;
        /// 无人机·空中飞行。它的下一个状态必须是隶属于`Motionless`组的状态
        #[derive(Builder, Debug, Default)]
        #[builder(pattern = "owned")]
        pub struct Flying<S>
        where S: Motionless {
            pub(super) origin: Coordinate,      // 【飞行状态】独有·起点字段
            pub(super) destination: Coordinate, // 【飞行状态】独有·终点字段
            #[builder(default = "Coordinate::step(1_f32)")]
            pub(super) step: Coordinate,        // 【飞行状态】独有·步长字段
            #[builder(setter(skip))]
            pub(super) handle: Option<JoinHandle<()>>,
            /// 零抽象成本的状态字段
            #[builder(setter(skip))]
            destination_state: PhantomData<S>
        }
        // 1. 限定【状态·类型】都必须实现`trait State`
        // 2. 禁止下游代码扩充新【状态·类型】
        seal_by_trait!(State, [Idle, Hovering], [Flying<S: Motionless>]);
        // 无人机·状态·分组：
        // 1. 静止的无人机 — 作为【飞行·状态】的过渡目标【状态】
        // 2. 空中的无人机 — 处于这类【状态】的【无人机】的拍照功能
        group_by_trait!(Motionless: State, [Idle, Hovering]);
        group_by_trait!(Midair: State, [Hovering], [Flying<S: Motionless>]);
    }
    /// 无人机·极坐标位置
    mod coordinate {
        use ::derive_builder::Builder;
        use ::std::fmt::{Display, Formatter, self};
        /// 无人机·位置坐标
        #[derive(Builder, Clone, Debug, Default, PartialEq, PartialOrd)]
        #[builder(default)]
        pub struct Coordinate {
            /// 经度
            pub(super) longitude: f32,
            /// 纬度
            pub(super) latitude: f32,
            /// 高度
            pub(super) altitude: f32
        }
        impl Coordinate {
            pub(super) fn step(value: f32) -> Self {
                Coordinate {
                    longitude: value,
                    latitude: value,
                    altitude: value
                }
            }
            pub(super) fn abs(&mut self) -> &mut Self {
                self.longitude = self.longitude.abs();
                self.latitude = self.latitude.abs();
                self.altitude = self.altitude.abs();
                self
            }
        }
        impl Display for Coordinate {
            fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
                write!(f, r#"坐标(经度："{}", 纬度："{}", 高度："{}")"#, self.longitude, self.latitude, self.altitude)
            }
        }
    }
    /// 模拟【无人机】缓慢飞行过程的【迭代器】
    mod flying_iterator {
        use ::derive_builder::Builder;
        use ::std::{iter::Iterator, sync::{Arc, Mutex, MutexGuard}};
        use super::Coordinate;
        /// 跟踪·无人机·的飞行位置
        #[derive(Builder, Debug, Default)]
        pub struct FlyingIter {
            /// 起点
            origin: Coordinate,
            /// 终点
            destination: Coordinate,
            /// 当前位置
            current: Arc<Mutex<Coordinate>>,
            /// 移动步长值
            step: Coordinate
        }
        impl Iterator for FlyingIter {
            type Item = Coordinate;
            fn next(&mut self) -> Option<Self::Item> {
                macro_rules! translate {
                    (tweak $component: ident, $current: expr) => {
                        if self.origin.$component < self.destination.$component {
                            let tmp = $current.$component + self.step.$component;
                            if tmp < self.destination.$component {
                                tmp
                            } else {
                                self.destination.$component
                            }
                        } else if self.origin.$component > self.destination.$component {
                            let tmp = $current.$component - self.step.$component;
                            if tmp > self.destination.$component {
                                tmp
                            } else {
                                self.destination.$component
                            }
                        } else {
                            self.destination.$component
                        }
                    }
                }
                get_mutex_lock!(self.current, |coord: &mut MutexGuard<Coordinate>| -> Option<Coordinate> {
                    coord.longitude = translate!(tweak longitude, coord);
                    coord.latitude = translate!(tweak latitude, coord);
                    coord.altitude = translate!(tweak altitude, coord);
                    if **coord == self.destination {
                        return None;
                    }
                    Some((*coord).clone())
                })
            }
        }
    }
    use ::async_std::task;
    use ::std::{sync::{Arc, Mutex, MutexGuard}, time::Duration};
    use coordinate::Coordinate;
    use flying_iterator::FlyingIterBuilder;
    use drone_states::{Flying, FlyingBuilder, Hovering, Midair, Motionless, State};
    pub use coordinate::CoordinateBuilder;
    pub use drone_states::Idle;
    /// 无人机·泛型类型
    /// 1. 最低内存成本的·按（普通）【引用】保存坐标位置`coordinate`
    /// 2. 不接收“下游”代码扩充的【状态·类型】`State`
    pub struct Drone<S>
    where S: State {
        /// 所有状态共有的坐标字段
        coordinate: Arc<Mutex<Coordinate>>,
        state: S,
    }
    /// 所有状态共有的成员方法
    impl<S> Drone<S>
    where S: State {
        /// 获取·无人机·此时此刻的坐标位置【快照】
        pub fn coordinate(&self) -> Coordinate {
            get_mutex_lock!(self.coordinate, |coord: &mut MutexGuard<Coordinate>| {
                coord.clone()
            })
        }
    }
    /// Idle - 无人机·在地面上
    ///
    /// 【待命】状态独有的【关联函数】与【成员方法】
    impl Drone<Idle> {
        /// 所有【新】无人机都得从【待命】状态开始，因为无人机的其它状态
        /// 都没有【构造函数】。
        pub fn new(mut coordinate: Coordinate) -> Self {
            if coordinate.altitude != 0_f32 {
                #[cfg(debug_assertions)]
                println!("无人机的出生地必须在地面上，所以将忽略高度值 {}", coordinate.altitude);
                coordinate.altitude = 0_f32;
            }
            Self {
                coordinate: Arc::new(Mutex::new(coordinate)),
                state: Idle
            }
        }
        /// 【起飞 - 状态·过渡】无人机·从地面到空中
        pub fn take_off(self, altitude: f32) -> Drone<Flying<Hovering>> {
            let origin = self.coordinate();
            let mut destination = origin.clone();
            destination.altitude = altitude;
            Drone {
                coordinate: self.coordinate,
                state: FlyingBuilder::default()
                    .origin(origin)
                    .destination(destination)
                    .build().unwrap()
            }
        }
    }
    /// Hovering - 无人机·原地悬浮于空中
    ///
    /// 【悬浮】状态独有的成员方法
    impl Drone<Hovering> {
        /// 【着落 - 状态·过渡】无人机·从空中到地面
        pub fn land(self) -> Drone<Flying<Idle>> {
            let origin = self.coordinate();
            let mut destination = origin.clone();
            destination.altitude = 0_f32;
            Drone {
                coordinate: self.coordinate,
                state: FlyingBuilder::default()
                    .origin(origin)
                    .destination(destination)
                    .build().unwrap()
            }
        }
        // 【飘移 - 状态·过渡】无人机·在空中从一处飞行到另一处
        pub fn move_to(self, destination: Coordinate) -> Drone<Flying<Hovering>> {
            let origin = self.coordinate();
            Drone {
                coordinate: self.coordinate,
                state: FlyingBuilder::default()
                    .origin(origin)
                    .destination(destination)
                    .build().unwrap()
            }
        }
    }
    /// Flying<S: Motionless> - 无人机·空中飞行。它的下一个状态必须是隶属于`Motionless`组的状态
    ///
    /// 【飞行】状态独有的成员方法
    impl<S> Drone<Flying<S>>
    where S: Motionless {
        async fn inner_fly(mut self, state: S, step: Option<Coordinate>) -> Drone<S>
        where S: Motionless {
            if self.state.handle.is_none() {
                if let Some(mut step) = step {
                    step.abs();
                    self.state.step = step;
                }
                let mut move_iter = FlyingIterBuilder::default()
                    .origin(self.state.origin)
                    .destination(self.state.destination)
                    .step(self.state.step)
                    .current(Arc::clone(&self.coordinate))
                    .build().unwrap();
                self.state.handle.replace(task::spawn(async move {
                    while let Some(_) = move_iter.next() {
                        task::sleep(Duration::from_millis(200)).await;
                    }
                }));
            } else if let Some(step) = step {
                #[cfg(debug_assertions)]
                println!("因一旦开始飞行就不能再修改步长值了，但是忽略了此值 {}", step);
            }
            self.state.handle.unwrap().await;
            Drone {
                coordinate: self.coordinate,
                state
            }
        }
    }
    /// 面向【着落】的【飞行】状态的独有成员方法
    impl Drone<Flying<Idle>> {
        pub async fn fly(self, step: Option<Coordinate>) -> Drone<Idle> {
            self.inner_fly(Idle, step).await
        }
    }
    /// 以【悬浮】为下一状态的【飞行】状态独有成员方法
    impl Drone<Flying<Hovering>> {
        pub async fn fly(self, step: Option<Coordinate>) -> Drone<Hovering> {
            self.inner_fly(Hovering, step).await
        }
    }
    /// 空中的【无人机】独有成员方法
    impl<S> Drone<S>
    where S: Midair {
        /// 拍照
        pub fn take_picture(&self) {
            println!("拍照一张在{}", self.coordinate());
        }
    }
}
use ::async_std::task;
use ::std::error::Error;
use drone_model::{CoordinateBuilder, Drone, Idle};
fn main() -> Result<(), Box<dyn Error>> {
    task::block_on(async {
        // 在地面上放一架【待命】模式的【无人机】
        let idle_drone1 = Drone::<Idle>::new(CoordinateBuilder::default()
            .longitude(15_f32)
            .latitude(12_f32)
            .altitude(2_f32)
            .build()?);
        #[cfg(debug_assertions)]
        println!("【待命·状态】无人机·被摆于地面{}。", idle_drone1.coordinate());
        // 命令【无人机】原地起飞，和指定拉升高度
        let flying_drone1 = idle_drone1.take_off(10_f32);
        #[cfg(debug_assertions)]
        println!("【飞行·状态】无人机·正在升空。拉升是个过程，所以这里用“异步函数”来模拟。");
        // 【无人机】拉升至 10 米高度，进入【悬浮】模式
        let hovering_drone1 = flying_drone1.fly(Some(CoordinateBuilder::default()
            .altitude(0.5_f32)
            .build()?)).await;
        hovering_drone1.take_picture();
        #[cfg(debug_assertions)]
        println!("【悬浮·状态】无人机·静止于空中{}。", hovering_drone1.coordinate());
        // 命令【无人机】转入巡航模式，和指定巡航目的地坐标
        let flying_drone2 = hovering_drone1.move_to(CoordinateBuilder::default()
            .longitude(30_f32)
            .latitude(20_f32)
            .altitude(5_f32)
            .build()?);
        flying_drone2.take_picture();
        #[cfg(debug_assertions)]
        println!("【飞行·状态】无人机·正在飞往指定空域。飞行是个过程，所以这里用“异步函数”来模拟。");
        // 【无人机】飞行至目的地，再次进入【悬浮】模式
        let hovering_drone2 = flying_drone2.fly(Some(CoordinateBuilder::default()
            .longitude(1.1)
            .latitude(1.2)
            .altitude(0.5)
            .build()?)).await;
        #[cfg(debug_assertions)]
        println!("【悬浮·状态】无人机·静止于空中{}。", hovering_drone2.coordinate());
        // 命令【无人机】原地着落
        let flying_drone2 = hovering_drone2.land();
        // 【无人机】安全着落，两次进入待命模式
        let idle_drone2 = flying_drone2.fly(Some(CoordinateBuilder::default()
            .altitude(0.6_f32)
            .build()?)).await;
        #[cfg(debug_assertions)]
        println!("【待命·状态】无人机·着落于地面{}。", idle_drone2.coordinate());
        Ok(())
    })
}
