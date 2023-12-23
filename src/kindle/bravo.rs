use std::collections::HashMap;

use chrono::Datelike;
use image::{GrayImage, Luma};
use imageproc::{drawing, rect::Rect};
use once_cell::sync::Lazy;
use rusttype::Scale;

use super::shared::*;
use anyhow::{anyhow, Result};

static MAP_WEEKDAY: Lazy<HashMap<u8, &'static str>> = Lazy::new(|| {
    let mut map = HashMap::new();
    map.insert(0, "周日");
    map.insert(1, "周一");
    map.insert(2, "周二");
    map.insert(3, "周三");
    map.insert(4, "周四");
    map.insert(5, "周五");
    map.insert(6, "周六");
    map.insert(7, "周日");
    map
});

pub fn generate(context: &Context) -> Result<GrayImage> {
    let font = context
        .fonts
        .get("main")
        .ok_or(anyhow!("main font not found"))?;
    let now = context.now.ok_or(anyhow!("time not provided"))?;
    static MAP: Lazy<HashMap<u8, &'static str>> = Lazy::new(|| {
        let mut map = HashMap::new();
        map.insert(1, "壹");
        map.insert(2, "贰");
        map.insert(3, "叁");
        map.insert(4, "肆");
        map.insert(5, "伍");
        map.insert(6, "陆");
        map.insert(7, "柒");
        map.insert(8, "捌");
        map.insert(9, "玖");
        map.insert(10, "拾");
        map.insert(20, "廿");
        map.insert(30, "卅");
        map
    });
    let mut img = GrayImage::new(600, 800);

    let rect = Rect::at(0, 0).of_size(img.width(), img.height());
    drawing::draw_filled_rect_mut(&mut img, rect, Luma([255]));

    let font_scale = Scale::uniform(375.0);
    let color = Luma([0]);
    let base = (40, 300);
    if now.day() < 10 {
        draw_aligned_text(
            &mut img,
            color,
            base,
            font_scale,
            font,
            MAP.get(&(now.day() as u8)).unwrap(),
            (AlignHorizontal::Left, AlignVertical::Center),
        );
    } else if now.day() == 10 {
        draw_aligned_text(
            &mut img,
            color,
            base,
            font_scale,
            font,
            MAP.get(&(10)).unwrap(),
            (AlignHorizontal::Left, AlignVertical::Center),
        );
    } else if (now.day() == 20) || (now.day() == 30) {
        draw_aligned_text(
            &mut img,
            color,
            base,
            font_scale,
            font,
            MAP.get(&((now.day() / 10) as u8)).unwrap(),
            (AlignHorizontal::Left, AlignVertical::Bottom),
        );
        draw_aligned_text(
            &mut img,
            color,
            base,
            font_scale,
            font,
            MAP.get(&10).unwrap(),
            (AlignHorizontal::Left, AlignVertical::Top),
        );
    } else {
        draw_aligned_text(
            &mut img,
            color,
            base,
            font_scale,
            font,
            MAP.get(&((now.day() / 10 * 10) as u8)).unwrap(),
            (AlignHorizontal::Left, AlignVertical::Bottom),
        );
        draw_aligned_text(
            &mut img,
            color,
            base,
            font_scale,
            font,
            MAP.get(&((now.day() % 10) as u8)).unwrap(),
            (AlignHorizontal::Left, AlignVertical::Top),
        );
    }
    draw_aligned_text(
        &mut img,
        color,
        (600 - 40, 20),
        Scale::uniform(150.0),
        font,
        &format!(
            "{}",
            MAP_WEEKDAY
                .get(&(now.weekday().num_days_from_sunday() as u8))
                .unwrap()
        ),
        (AlignHorizontal::Right, AlignVertical::Top),
    );

    return Ok(img);
}