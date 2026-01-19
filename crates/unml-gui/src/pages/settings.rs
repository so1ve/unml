mod java;
mod view;

pub use java::init as init_java_settings;
pub use view::page;

unml_macros::define_sidebar! {
    variant: Navigation,

    section {
        General => "settings.general",
        Java => "settings.java",
        Game => "settings.game",
        Download => "settings.download",
        About => "settings.about",
    }
}
