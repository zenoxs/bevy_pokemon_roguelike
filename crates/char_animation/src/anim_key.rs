use bevy::reflect::Reflect;
use serde::{Deserialize, Serialize};
use strum::{Display, IntoStaticStr};

#[derive(
    Debug,
    Reflect,
    IntoStaticStr,
    Default,
    Display,
    Deserialize,
    Serialize,
    Eq,
    PartialEq,
    Hash,
    Clone,
    Copy,
)]
pub enum AnimKey {
    Walk,
    Attack,
    Kick,
    Shoot,
    Strike,
    Sleep,
    Hurt,
    #[default]
    Idle,
    Swing,
    Double,
    Hop,
    Charge,
    Rotate,
    EventSleep,
    Wake,
    Eat,
    Tumble,
    Pose,
    Pull,
    Pain,
    Float,
    DeepBreath,
    Nod,
    Sit,
    LookUp,
    Sink,
    Trip,
    Laying,
    LeapForth,
    Head,
    Cringe,
    LostBalance,
    TumbleBack,
    TailWhip,
    Faint,
    HitGround,
    Dance,
    Shake,
    SpAttack,
    Twirl,
    Withdraw,
    Ricochet,
}
