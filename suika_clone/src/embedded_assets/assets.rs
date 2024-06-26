use bevy::{
    prelude::Plugin,
    asset::embedded_asset,
};


pub struct ScEmbeddedAssetsPlugin;


impl Plugin for ScEmbeddedAssetsPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        embedded_asset!(app, "fonts/x12y12pxMaruMinyaM.ttf");

        embedded_asset!(app, "ron/kao.game.ron");

        embedded_asset!(app, "images/kao/kao_01.png");
        embedded_asset!(app, "images/kao/kao_02.png");
        embedded_asset!(app, "images/kao/kao_03.png");
        embedded_asset!(app, "images/kao/kao_04.png");
        embedded_asset!(app, "images/kao/kao_05.png");
        embedded_asset!(app, "images/kao/kao_06.png");
        embedded_asset!(app, "images/kao/kao_07.png");
        embedded_asset!(app, "images/kao/kao_08.png");
        embedded_asset!(app, "images/kao/kao_09.png");
        embedded_asset!(app, "images/kao/kao_10.png");
        embedded_asset!(app, "images/kao/kao_11.png");

        embedded_asset!(app, "images/kao/player.png");

        embedded_asset!(app, "images/effects/effect_small_1.png");
        embedded_asset!(app, "images/effects/effect_small_2.png");
        embedded_asset!(app, "images/effects/effect_small_3.png");

        embedded_asset!(app, "images/effects/circle_16x16.png");
        embedded_asset!(app, "images/effects/rectangle_16x16.png");
        embedded_asset!(app, "images/effects/buzz_16x16.png");

        embedded_asset!(app, "images/bottle_fg_toriaezu.png");
        embedded_asset!(app, "images/bottle_bg_toriaezu.png");

        embedded_asset!(app, "images/bg_toriaezu_1280x840.png");
        embedded_asset!(app, "images/bg_aozora_1280x840.png");
        embedded_asset!(app, "images/border1.png");
        embedded_asset!(app, "images/border2.png");
        embedded_asset!(app, "images/popup2_128x128_32.png");
        embedded_asset!(app, "images/popup3_128x128_32.png");

        embedded_asset!(app, "images/title_1280x840.png");

        embedded_asset!(app, "sounds/bgm.ogg");
        embedded_asset!(app, "sounds/se_combine.ogg");
    }
}

pub const DEFAULT_GAME_RON_PATH: &str = concat!(
    "embedded://",
    env!("CARGO_CRATE_NAME"),
    "/embedded_assets/ron/kao.game.ron",
);
