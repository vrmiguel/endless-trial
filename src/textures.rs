use std::time::Instant;

use arrayvec::ArrayVec;
use rand::{seq::SliceRandom, Rng};
use tetra::{graphics::Texture, Context};

use crate::{
    humanoid::HumanoidType,
    resources::{
        BADASS_GRUNTS, BASIC_GRUNTS, BOSS, STRONGER_GRUNTS,
    },
};

/// Loads all grunt sprites into memory to avoid
/// recreating the texture when spawning enemies
pub struct GruntTextures {
    basic_grunts: [Texture; BASIC_GRUNTS.len()],
    stronger_grunts: [Texture; STRONGER_GRUNTS.len()],
    badass_grunts: [Texture; BADASS_GRUNTS.len()],
    boss: Texture,
}

impl GruntTextures {
    fn load_textures<const N: usize>(
        ctx: &mut Context,
        sprites: &[&[u8]],
    ) -> [Texture; N] {
        let mut textures: ArrayVec<Texture, N> = ArrayVec::new();

        for sprite in sprites {
            textures.push(
                Texture::from_encoded(ctx, sprite).unwrap(),
            );
        }

        textures.into_inner().unwrap()
    }

    pub fn load(ctx: &mut Context) -> Self {
        let now = Instant::now();

        let textures = Self {
            basic_grunts: Self::load_textures(ctx, BASIC_GRUNTS),
            stronger_grunts: Self::load_textures(
                ctx,
                STRONGER_GRUNTS,
            ),
            badass_grunts: Self::load_textures(
                ctx,
                BADASS_GRUNTS,
            ),
            boss: Texture::from_encoded(ctx, BOSS).unwrap(),
        };

        println!(
            "Loaded all enemy textures into GPU memory in {}ms",
            now.elapsed().as_millis()
        );

        textures
    }

    /// Choose a random texture of the given enemy kind
    pub fn choose_enemy_from_kind<R: Rng>(
        &self,
        kind: HumanoidType,
        rng: &mut R,
    ) -> Texture {
        match kind {
            HumanoidType::Player => unreachable!(
                "An enemy cannot have the player's sprite"
            ),
            HumanoidType::BasicEnemy => {
                self.basic_grunts.choose(rng).unwrap()
            }
            HumanoidType::StrongerEnemy => {
                self.stronger_grunts.choose(rng).unwrap()
            }
            HumanoidType::BadassEnemy => {
                self.badass_grunts.choose(rng).unwrap()
            }
            HumanoidType::Boss => &self.boss,
        }
        .clone() // Texture is an Rc so this clone is cheap
    }
}
