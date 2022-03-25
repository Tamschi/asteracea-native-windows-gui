use asteracea::bumpalo::format;
use std::cell::RefCell;

asteracea::component! {
	/// A plain single-line text input.
	pub TextInput(
		text: String = "".to_string(),
	)(
		x?: u32,
		y?: u32,

		width?: u32,
		height?: u32,

		visible: bool = true,
		enabled: bool = true,

		placeholder_text?: &'bump str,

		focus: bool = false,

		limit?: usize,
	) -> !Sync

	let self.text = RefCell::<String>::new(text);

	<"nwg-TextInput"
	//TODO?: ."data-nwg-x"?=!"{}"(?x)
		."data-nwg-x"?={x.map(|x| format!(in bump, "{}", x).into_bump_str())}
		."data-nwg-y"?={y.map(|y| format!(in bump, "{}", y).into_bump_str())}

		."data-nwg-width"?={width.map(|width| format!(in bump, "{}", width).into_bump_str())}
		."data-nwg-height"?={height.map(|height| format!(in bump, "{}", height).into_bump_str())}

		."data-nwg-enabled"?={enabled}
		."data-nwg-visible"?={visible}

		."data-nwg-placeholder_text"?={placeholder_text}

		."data-nwg-focus"?={focus}

		."data-nwg-limit"?={limit.map(|limit| format!(in bump, "{}", limit).into_bump_str())}

		on bubble "nwg-OnTextInput" = fn(self, _event) {
			todo!()
		}

		!"{}"(self.text.borrow())
	>
}
