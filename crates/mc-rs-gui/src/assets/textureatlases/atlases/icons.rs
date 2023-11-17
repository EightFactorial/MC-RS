use mc_rs_macros::impl_atlasdata;

use crate::assets::textureatlases::TextureAtlasType;

impl_atlasdata! {
    IconAtlas,
    "minecraft:gui/icons",
    TextureAtlasType::Icons,
    CROSSHAIR = [0, 0, 15, 15],

    PLAYER_HEART_OUTLINE_0 = [15, 0, 25, 9],
    PLAYER_HEART_OUTLINE_1 = [25, 0, 34, 9],
    PLAYER_HEART_OUTLINE_2 = [34, 0, 43, 9],
    PLAYER_HEART_OUTLINE_3 = [43, 0, 52, 9],

    PLAYER_HEART_0_FULL = [52, 0, 61, 9],
    PLAYER_HEART_0_HALF = [61, 0, 70, 9],
    PLAYER_HEART_1_FULL = [70, 0, 79, 9],
    PLAYER_HEART_1_HALF = [79, 0, 88, 9],
    PLAYER_HEART_2_FULL = [88, 0, 97, 9],
    PLAYER_HEART_2_HALF = [97, 0, 106, 9],
    PLAYER_HEART_3_FULL = [106, 0, 115, 9],
    PLAYER_HEART_3_HALF = [115, 0, 124, 9],
    PLAYER_HEART_4_FULL = [124, 0, 133, 9],
    PLAYER_HEART_4_HALF = [133, 0, 142, 9],
    PLAYER_HEART_5_FULL = [142, 0, 151, 9],
    PLAYER_HEART_5_HALF = [151, 0, 160, 9],
    PLAYER_HEART_6_FULL = [160, 0, 169, 9],
    PLAYER_HEART_6_HALF = [169, 0, 178, 9],
    PLAYER_HEART_7_FULL = [178, 0, 187, 9],
    PLAYER_HEART_7_HALF = [187, 0, 196, 9],

    PLAYER_ARMOR_EMPTY = [15, 9, 25, 18],
    PLAYER_ARMOR_HALF = [25, 9, 34, 18],
    PLAYER_ARMOR_FULL_0 = [34, 9, 43, 18],
    PLAYER_ARMOR_FULL_1 = [43, 9, 52, 18],

    ANIMAL_HEART_OUTLINE_0 = [52, 9, 61, 18],
    ANIMAL_HEART_OUTLINE_1 = [61, 9, 70, 18],
    ANIMAL_HEART_OUTLINE_2 = [70, 9, 79, 18],
    ANIMAL_HEART_OUTLINE_3 = [79, 9, 88, 18],

    ANIMAL_HEART_0_FULL = [88, 9, 97, 18],
    ANIMAL_HEART_0_HALF = [97, 9, 106, 18],
    ANIMAL_HEART_1_FULL = [106, 9, 115, 18],
    ANIMAL_HEART_1_HALF = [115, 9, 124, 18],

    PLAYER_BREATH_BUBBLE = [15, 18, 25, 27],
    PLAYER_BREATH_POP = [25, 18, 34, 27],

    PLAYER_ARMOR_WATER_HALF = [34, 18, 43, 27],
    PLAYER_ARMOR_WATER = [43, 18, 52, 27],

    PLAYER_FOOD_OUTLINE_0 = [15, 27, 25, 36],
    PLAYER_FOOD_OUTLINE_1 = [25, 27, 34, 36],
    PLAYER_FOOD_OUTLINE_2 = [34, 27, 43, 36],
    PLAYER_FOOD_OUTLINE_3 = [43, 27, 52, 36],

    PLAYER_FOOD_0_FULL = [52, 27, 61, 36],
    PLAYER_FOOD_0_HALF = [61, 27, 70, 36],
    PLAYER_FOOD_1_FULL = [70, 27, 79, 36],
    PLAYER_FOOD_1_HALF = [79, 27, 88, 36],
    PLAYER_FOOD_2_FULL = [88, 27, 97, 36],
    PLAYER_FOOD_2_HALF = [97, 27, 106, 36],
    PLAYER_FOOD_3_FULL = [106, 27, 115, 36],
    PLAYER_FOOD_3_HALF = [115, 27, 124, 36],

    PLAYER_FOOD_OUTLINE_4 = [124, 27, 133, 36],
    PLAYER_FOOD_OUTLINE_5 = [133, 27, 142, 36],

    PLAYER_FOOD_FLIPPED = [15, 36, 24, 45],

    PLAYER_HARDCORE_HEART_OUTLINE_0 = [15, 45, 25, 54],
    PLAYER_HARDCORE_HEART_OUTLINE_1 = [25, 45, 34, 54],
    PLAYER_HARDCORE_HEART_OUTLINE_2 = [34, 45, 43, 54],
    PLAYER_HARDCORE_HEART_OUTLINE_3 = [43, 45, 52, 54],

    PLAYER_HARDCORE_HEART_0_FULL = [52, 45, 61, 54],
    PLAYER_HARDCORE_HEART_0_HALF = [61, 45, 70, 54],
    PLAYER_HARDCORE_HEART_1_FULL = [70, 45, 79, 54],
    PLAYER_HARDCORE_HEART_1_HALF = [79, 45, 88, 54],
    PLAYER_HARDCORE_HEART_2_FULL = [88, 45, 97, 54],
    PLAYER_HARDCORE_HEART_2_HALF = [97, 45, 106, 54],
    PLAYER_HARDCORE_HEART_3_FULL = [106, 45, 115, 54],
    PLAYER_HARDCORE_HEART_3_HALF = [115, 45, 124, 54],
    PLAYER_HARDCORE_HEART_4_FULL = [124, 45, 133, 54],
    PLAYER_HARDCORE_HEART_4_HALF = [133, 45, 142, 54],
    PLAYER_HARDCORE_HEART_5_FULL = [142, 45, 151, 54],
    PLAYER_HARDCORE_HEART_5_HALF = [151, 45, 160, 54],
    PLAYER_HARDCORE_HEART_6_FULL = [160, 45, 169, 54],
    PLAYER_HARDCORE_HEART_6_HALF = [169, 45, 178, 54],
    PLAYER_HARDCORE_HEART_7_FULL = [178, 45, 187, 54],
    PLAYER_HARDCORE_HEART_7_HALF = [187, 45, 196, 54],

    CONNECTION_STRENGTH_5 = [0, 16, 10, 24],
    CONNECTION_STRENGTH_4 = [0, 24, 10, 32],
    CONNECTION_STRENGTH_3 = [0, 32, 10, 40],
    CONNECTION_STRENGTH_2 = [0, 40, 10, 48],
    CONNECTION_STRENGTH_1 = [0, 48, 10, 56],
    CONNECTION_STRENGTH_0 = [0, 56, 10, 64],

    XP_BAR_BACKGROUND = [0, 64, 182, 69],
    XP_BAR_FOREGROUND = [0, 69, 182, 74],
    BOSS_BAR_BACKGROUND = [0, 74, 182, 79],
    BOSS_BAR_FOREGROUND = [0, 79, 182, 84],
    HORSE_JUMP_BAR_BACKGROUND = [0, 84, 182, 89],
    HORSE_JUMP_BAR_FOREGROUND = [0, 89, 182, 94],

    WEAPON_SWING_0 = [0, 94, 18, 112],
    WEAPON_SWING_1 = [18, 94, 36, 112],
    WEAPON_SWING_2 = [36, 94, 52, 102],
    WEAPON_SWING_3 = [52, 94, 68, 102],
    WEAPON_SWING_4 = [68, 94, 84, 102],

    SERVER_PING_STRENGTH_5 = [0, 176, 10, 184],
    SERVER_PING_STRENGTH_4 = [0, 184, 10, 192],
    SERVER_PING_STRENGTH_3 = [0, 192, 10, 200],
    SERVER_PING_STRENGTH_2 = [0, 200, 10, 208],
    SERVER_PING_STRENGTH_1 = [0, 208, 10, 216],
    SERVER_PING_STRENGTH_0 = [0, 216, 10, 224],

    SERVER_PING_ANIM_0 = [10, 176, 20, 184],
    SERVER_PING_ANIM_1 = [10, 184, 20, 192],
    SERVER_PING_ANIM_2 = [10, 192, 20, 200],
    SERVER_PING_ANIM_3 = [10, 200, 20, 208],
    SERVER_PING_ANIM_4 = [10, 208, 20, 216],
}
