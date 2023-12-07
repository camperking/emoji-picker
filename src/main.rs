use arboard::Clipboard;
use clap::{Parser, ValueEnum};
use emojis::Emoji;
use vizia::prelude::*;

#[derive(Clone, Debug, ValueEnum)]
enum Theme {
    System,
    Light,
    Dark,
}

/// Emoji Picker
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    // /// theme to use, light or dark, if not specified, system theme will be used
    #[arg(short, long, default_value = "system")]
    theme: Theme,
}

#[derive(Clone, Data, PartialEq)]
pub enum Group {
    SmileysAndEmotion,
    PeopleAndBody,
    AnimalsAndNature,
    FoodAndDrink,
    TravelAndPlaces,
    Activities,
    Objects,
    Symbols,
    Flags,
}

fn group_to_emoji_group(group: &Group) -> emojis::Group {
    match group {
        Group::SmileysAndEmotion => emojis::Group::SmileysAndEmotion,
        Group::PeopleAndBody => emojis::Group::PeopleAndBody,
        Group::AnimalsAndNature => emojis::Group::AnimalsAndNature,
        Group::FoodAndDrink => emojis::Group::FoodAndDrink,
        Group::TravelAndPlaces => emojis::Group::TravelAndPlaces,
        Group::Activities => emojis::Group::Activities,
        Group::Objects => emojis::Group::Objects,
        Group::Symbols => emojis::Group::Symbols,
        Group::Flags => emojis::Group::Flags,
    }
}

#[derive(Clone, Data, Debug, PartialEq)]
pub enum SkinTone {
    Default,
    Light,
    MediumLight,
    Medium,
    MediumDark,
    Dark,
}

fn skin_tone_to_emoji_skin_tone(skin_tone: &SkinTone) -> emojis::SkinTone {
    match skin_tone {
        SkinTone::Default => emojis::SkinTone::Default,
        SkinTone::Light => emojis::SkinTone::Light,
        SkinTone::MediumLight => emojis::SkinTone::MediumLight,
        SkinTone::Medium => emojis::SkinTone::Medium,
        SkinTone::MediumDark => emojis::SkinTone::MediumDark,
        SkinTone::Dark => emojis::SkinTone::Dark,
    }
}

#[derive(Lens)]
pub struct AppData {
    search: String,
    group: Group,
    skin_tone: SkinTone,
    filter: (Group, SkinTone, String),
    clipboard: Clipboard,
}

pub enum AppEvent<'e> {
    Search(String),
    Group(Group),
    SkinTone(SkinTone),
    Clipboard(&'e Emoji),
}

impl Model for AppData {
    fn event(&mut self, _cx: &mut EventContext, event: &mut Event) {
        event.map(|app_event, _meta| match app_event {
            AppEvent::Search(search) => {
                self.search = search.to_string();
                self.filter = (
                    self.group.clone(),
                    self.skin_tone.clone(),
                    search.to_string(),
                );
            }
            AppEvent::Group(group) => {
                self.group = group.clone();
                self.filter = (group.clone(), self.skin_tone.clone(), self.search.clone());
            }
            AppEvent::SkinTone(skin_tone) => {
                self.skin_tone = skin_tone.clone();
                self.filter = (self.group.clone(), skin_tone.clone(), self.search.clone());
            }
            AppEvent::Clipboard(emoji) => {
                self.clipboard.set_text(emoji.as_str()).unwrap();
            }
        });
    }
}

fn main() {
    Application::new(|cx| {
        let args = Args::parse();

        match args.theme {
            Theme::System => {
                cx.emit(EnvironmentEvent::SetThemeMode(AppTheme::System));
            }
            Theme::Light => {
                cx.emit(EnvironmentEvent::SetThemeMode(AppTheme::BuiltIn(
                    ThemeMode::LightMode,
                )));
            }
            Theme::Dark => {
                cx.emit(EnvironmentEvent::SetThemeMode(AppTheme::BuiltIn(
                    ThemeMode::DarkMode,
                )));
            }
        }

        cx.add_font_mem(include_bytes!("../assets/NotoColorEmoji-Regular.ttf"));
        cx.set_default_font(&["Noto Color Emoji"]);

        AppData {
            search: String::new(),
            group: Group::SmileysAndEmotion,
            skin_tone: SkinTone::Default,
            filter: (Group::SmileysAndEmotion, SkinTone::Default, String::new()),
            clipboard: Clipboard::new().unwrap(),
        }
        .build(cx);

        VStack::new(cx, |cx| {
            HStack::new(cx, |cx| {
                Textbox::new(cx, AppData::search)
                    .placeholder("Search")
                    .on_edit(|cx, search| cx.emit(AppEvent::Search(search)))
                    .width(Stretch(1.0));

                Dropdown::new(
                    cx,
                    |cx| Label::new(cx, "🖖"),
                    |cx| {
                        fn dropdown_item<'a>(
                            cx: &'a mut Context,
                            label: &'a str,
                            skin_tone: SkinTone,
                        ) -> Handle<'a, Label> {
                            Label::new(cx, label)
                                .on_press(move |cx| {
                                    cx.emit(AppEvent::SkinTone(skin_tone.clone()));
                                    cx.emit(PopupEvent::Close);
                                })
                                .child_space(Pixels(8.0))
                                .on_hover(|cx| {
                                    cx.set_background_color(Color::rgba(0, 0, 0, 10));
                                })
                                .on_hover_out(|cx| {
                                    cx.set_background_color(Color::rgba(0, 0, 0, 0));
                                })
                        }

                        dropdown_item(cx, "🖖", SkinTone::Default);
                        dropdown_item(cx, "🖖🏻", SkinTone::Light);
                        dropdown_item(cx, "🖖🏼", SkinTone::MediumLight);
                        dropdown_item(cx, "🖖🏽", SkinTone::Medium);
                        dropdown_item(cx, "🖖🏾", SkinTone::MediumDark);
                        dropdown_item(cx, "🖖🏿", SkinTone::Dark);
                    },
                )
                .max_width(Pixels(38.0));
            })
            .width(Stretch(1.0))
            .height(Auto);

            HStack::new(cx, |cx| {
                fn group_button<'a>(
                    cx: &'a mut Context,
                    group: Group,
                    emoji: &'a str,
                ) -> Handle<'a, Button> {
                    let group_clone = group.clone();
                    Button::new(
                        cx,
                        move |cx| {
                            let group = group_clone.clone();
                            cx.emit(AppEvent::Group(group))
                        },
                        |cx| Label::new(cx, emoji),
                    )
                    .tooltip(|cx| {
                        Label::new(
                            cx,
                            match group {
                                Group::SmileysAndEmotion => "Smileys & Emotion",
                                Group::PeopleAndBody => "People & Body",
                                Group::AnimalsAndNature => "Animals & Nature",
                                Group::FoodAndDrink => "Food & Drink",
                                Group::TravelAndPlaces => "Travel & Places",
                                Group::Activities => "Activities",
                                Group::Objects => "Objects",
                                Group::Symbols => "Symbols",
                                Group::Flags => "Flags",
                            },
                        );
                    })
                    .width(Stretch(1.0))
                }

                group_button(cx, Group::SmileysAndEmotion, "😊");
                group_button(cx, Group::PeopleAndBody, "👨‍👩‍👧‍👦");
                group_button(cx, Group::AnimalsAndNature, "🐶🌳");
                group_button(cx, Group::FoodAndDrink, "🍔🍕");
                group_button(cx, Group::TravelAndPlaces, "✈️🗺️");
                group_button(cx, Group::Activities, "⚽🎮");
                group_button(cx, Group::Objects, "📷💻");
                group_button(cx, Group::Symbols, "❤️✨");
                group_button(cx, Group::Flags, "🏳️🚩");
            })
            .width(Stretch(1.0))
            .height(Auto);

            Binding::new(cx, AppData::filter, |cx, data| {
                ScrollView::new(cx, 0.0, 0.0, false, true, move |cx| {
                    let (group, skin_tone, search) = data.get(cx);
                    let iter = emojis::iter().filter(|e| {
                        let unicode_version =
                            e.unicode_version() < emojis::UnicodeVersion::new(15, 0);

                        if search.is_empty() {
                            unicode_version && e.group() == group_to_emoji_group(&group)
                        } else {
                            unicode_version
                                && e.name().contains(&search)
                                && search.chars().count() > 1
                        }
                    });

                    let items_per_row = 10;

                    let emojis: Vec<&emojis::Emoji> = iter.collect();

                    let mut row = emojis.len() / items_per_row;

                    if row == 0 {
                        row += 1;
                    }

                    for i in 0..row {
                        HStack::new(cx, |cx| {
                            for j in 0..items_per_row {
                                let index = i * items_per_row + j;

                                if index >= emojis.len() {
                                    break;
                                }

                                let emoji = emojis[index];

                                let emoji_toned =
                                    &emoji.with_skin_tone(skin_tone_to_emoji_skin_tone(&skin_tone));

                                let emoji = match emoji_toned {
                                    Some(emoji) => emoji,
                                    None => emoji,
                                };

                                Button::new(
                                    cx,
                                    |cx| cx.emit(AppEvent::Clipboard(emoji)),
                                    |cx| {
                                        Label::new(cx, &emoji.to_string()).font_size(24.0)
                                        // .width(Stretch(1.0))
                                    },
                                )
                                .tooltip(|cx| {
                                    Label::new(cx, emoji.name());
                                })
                                .width(Stretch(1.0))
                                .height(Pixels(48.0));
                            }
                        })
                        .width(Stretch(1.0))
                        .height(Auto);
                    }
                })
                .size(Stretch(1.0));
            });
        })
        .space(Pixels(10.0))
        .row_between(Pixels(10.0));
    })
    .title("Emoji Picker")
    .inner_size((640, 640))
    .resizable(false)
    .run();
}
