use rand::{Rand, Rng};

use error::Error;

#[derive(Debug)]
pub struct Hero {
    name: HeroName,
    role: Role,
    sub_role: SubRole,
}

#[derive(Copy, Clone, Debug, Deserialize, Serialize)]
pub enum HeroName {
    Abathur,
    Anubarak,
    Artanis,
    Arthas,
    Azmodan,
    Brightwing,
    Chen,
    Cho,
    Diablo,
    ETC,
    Falstad,
    Gall,
    Gazlowe,
    Illidan,
    Jaina,
    Johanna,
    Kaelthas,
    Kerrigan,
    Kharazim,
    Leoric,
    Lili,
    LtMorales,
    Lunara,
    Malfurion,
    Muradin,
    Murky,
    Nazeebo,
    Nova,
    Raynor,
    Rehgar,
    Rexxar,
    SgtHammer,
    Sonya,
    Stitches,
    Sylvanas,
    Tassadar,
    TheButcher,
    TheLostVikings,
    Thrall,
    Tychus,
    Tyrael,
    Tyrande,
    Uther,
    Valla,
    Zagara,
    Zeratul,
}

impl Rand for HeroName {
    fn rand<R: Rng>(rng: &mut R) -> Self {
        rng.choose(HEROES).expect("HEROES const was empty").name
    }
}

#[derive(Debug)]
pub enum Role {
    Assassin,
    Specialist,
    Support,
    Warrior,
}

#[derive(Debug)]
pub enum SubRole {
    Ambusher,
    Bruiser,
    BurstDamage,
    Healer,
    Siege,
    Support,
    SustainedDamage,
    Tank,
    Utility,
}

macro_rules! heroes {
    ($([$name:ident, $role:ident, $sub_role:ident]),+) => {
        pub const HEROES: &'static [Hero] = &[
            $(Hero { name: HeroName::$name, role: Role::$role, sub_role: SubRole::$sub_role },)*
        ];
    }
}

heroes!(
    [Abathur, Specialist, Utility],
    [Anubarak, Warrior, Bruiser],
    [Artanis, Warrior, Bruiser],
    [Arthas, Warrior, Bruiser],
    [Azmodan, Specialist, Siege],
    [Brightwing, Support, Healer],
    [Chen, Warrior, Tank],
    [Cho, Warrior, Tank],
    [Diablo, Warrior, Tank],
    [ETC, Warrior, Tank],
    [Falstad, Assassin, SustainedDamage],
    [Gall, Assassin, SustainedDamage],
    [Gazlowe, Specialist, Siege],
    [Illidan, Assassin, SustainedDamage],
    [Jaina, Assassin, BurstDamage],
    [Johanna, Warrior, Tank],
    [Kaelthas, Assassin, BurstDamage],
    [Kerrigan, Assassin, Ambusher],
    [Kharazim, Support, Healer],
    [Leoric, Warrior, Bruiser],
    [Lili, Support, Healer],
    [LtMorales, Support, Healer],
    [Lunara, Assassin, SustainedDamage],
    [Malfurion, Support, Healer],
    [Muradin, Warrior, Tank],
    [Murky, Specialist, Utility],
    [Nazeebo, Specialist, SustainedDamage],
    [Nova, Assassin, Ambusher],
    [Raynor, Assassin, SustainedDamage],
    [Rehgar, Support, Healer],
    [Rexxar, Warrior, Tank],
    [SgtHammer, Specialist, Siege],
    [Sonya, Warrior, Bruiser],
    [Stitches, Warrior, Tank],
    [Sylvanas, Specialist, Siege],
    [Tassadar, Support, Support],
    [TheButcher, Assassin, Ambusher],
    [TheLostVikings, Specialist, Utility],
    [Thrall, Assassin, SustainedDamage],
    [Tychus, Assassin, SustainedDamage],
    [Tyrael, Warrior, Bruiser],
    [Tyrande, Support, Support],
    [Uther, Support, Healer],
    [Valla, Assassin, SustainedDamage],
    [Zagara, Specialist, Siege],
    [Zeratul, Assassin, Ambusher]
);

