use amethyst::{
    assets::{AssetStorage, Loader},
    audio::{output::Output, AudioSink, OggFormat, Source, SourceHandle},
    ecs::prelude::World,
};

pub fn play_bounce(sounds: &Sounds, storage: &AssetStorage<Source>, output: Option<&Output>) {
    if let Some(output) = output.as_ref() {
        if let Some(sound) = storage.get(&sounds.bounce_sfx) {
            output.play_once(sound, 1.0);
        }
    }
}

pub struct Sounds {
    pub score_sfx: SourceHandle,
    pub bounce_sfx: SourceHandle,
}

fn load_audio_track(loader: &Loader, world: &World, file: &str) -> SourceHandle {
    loader.load(file, OggFormat, (), &world.read_resource())
}

pub fn initialize_audio(world: &mut World) {
    use crate::{AUDIO_BOUNCE, AUDIO_SCORE};


    let sound_effects = {
        let loader = world.read_resource::<Loader>();

        Sounds {
            bounce_sfx: load_audio_track(&loader, world, AUDIO_BOUNCE),
            score_sfx: load_audio_track(&loader, world, AUDIO_BOUNCE),
        }
    };

    world.add_resource(sound_effects);
}