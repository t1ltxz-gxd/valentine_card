use colored::*;
#[allow(unused_imports)]
use std::str::FromStr;

fn parse_rgb(color: &str) -> Option<(u8, u8, u8)> {
	let parts: Vec<&str> = color
		.trim_matches(|c| c == '(' || c == ')' || c == ' ')
		.split([',', ' '])
		.collect();
	if parts.len() == 3 {
		if let (Ok(r), Ok(g), Ok(b)) = (
			parts[0].trim().parse(),
			parts[1].trim().parse(),
			parts[2].trim().parse(),
		) {
			return Some((r, g, b));
		}
	}
	None
}

pub fn heart_builder(name: &str, color: Option<&str>) -> ColoredString {
	let mut heart = "".to_owned().normal();
	let color = color.and_then(|c| {
		if let Some(rgb) = parse_rgb(c) {
			Some(Color::TrueColor {
				r: rgb.0,
				g: rgb.1,
				b: rgb.2,
			})
		} else {
			Color::from_str(c).ok()
		}
	});

	let name_chars: Vec<char> = name.chars().collect();
	let name_len = name_chars.len();
	let mut name_idx = 0;

	for y in (-15..15).rev() {
		for x in -30..30 {
			let formula = (y as f64 * 0.1).mul_add(y as f64 * 0.1, (x as f64 * 0.05).powi(2)) - 1.0;
			if (x as f64 * 0.05)
				.powi(2)
				.mul_add(-(y as f64 * 0.1).powi(3), formula.powi(3))
				<= 0.0
			{
				let mut ch = name_chars[name_idx % name_len].to_string();
				if let Some(color) = color {
					ch = ch.color(color).to_string();
				}
				heart = ColoredString::from(heart.to_string() + &ch);
				name_idx += 1;
			} else {
				heart = ColoredString::from(heart.to_string() + " ");
			}
		}
		heart = ColoredString::from(heart.to_string() + "\n");
	}
	heart
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn parse_rgb_valid_comma_separated() {
		assert_eq!(parse_rgb("255,255,255"), Some((255, 255, 255)));
	}

	#[test]
	fn parse_rgb_valid_space_separated() {
		assert_eq!(parse_rgb("255 255 255"), Some((255, 255, 255)));
	}

	#[test]
	fn parse_rgb_invalid_format() {
		assert_eq!(parse_rgb("255,255"), None);
	}

	#[test]
	fn parse_rgb_non_numeric() {
		assert_eq!(parse_rgb("abc,255,255"), None);
	}

	#[test]
	fn heart_builder_with_invalid_rgb_color() {
		let result = heart_builder("Love", Some("invalid"));
		assert!(!result.contains("\u{1b}[38;2;"));
	}

	#[test]
	fn heart_builder_without_color() {
		let result = heart_builder("Love", None);
		assert!(!result.contains("\u{1b}[38;2;"));
	}
}
