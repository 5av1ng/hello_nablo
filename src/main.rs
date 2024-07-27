use nablo::prelude::*;
const SHADER: &str = r#"
@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
	if uniforms.info == 0u {
		if in.is_texture == 0u {
			return in.color;
		}else {
			return textureSample(t_diffuse, s_diffuse, in.color.xy);
		}
	}
	let loop_times = 5;
	var uv = (in.clip_position.xy + uniforms.mouse_position) / uniforms.window_xy;
	var out =  vec3f(0.0);
	var i = 0;
	var j = 1;
	loop {
		loop {
			out = out + scale(fract(uv * f32(j)), 1.0, vec2f(0.5)) / f32(j) / 1.5;
			j += 1;
			if j > loop_times {
				break;
			}
		}
		i += 1;
		if i > loop_times {
			break;
		}
	}
	return vec4f(out, 1.0);
}

fn compress(input: f32, p: f32) -> f32 {
	var max = 1.0;
	var in = abs(input);
	if (in > max) {
		return max;
	}else {
		return (pow(in, p) + pow(in, p)) / 2.0 * max;
	}

}

fn scale(in_uv: vec2f, factor: f32, offset: vec2f) -> vec3f {
	var uv = (in_uv - offset) / factor;
	var sined = abs(sin(p_norm(uv, 0.9 + 0.5 * sin(uniforms.time)) * 4.0 + uniforms.time * 0.5));
	var out = compress(pow(sined, 100.0), 10.0);
	return vec3(out * 1.0,out * (0.5 * sin(uniforms.time / 50.0) + 0.5 ) * 2.0,out * 5.0 * abs(sin(uniforms.time)));
}

fn p_norm(in: vec2f, p: f32) -> f32 {
	return pow(pow(in.x, p) + pow(in.y, p), 1.0 / p);
}
"#;

#[derive(Debug, Default)]
struct Main {
	some_value: f32,
	status: Status,
	some_text: String,
	block_close_button: bool,
	is_shader_on: bool
}

impl App for Main {
	fn on_open(&mut self, ui: &mut Ui) {
		ui.registrate_shader("wow", SHADER.to_string());
	}

	fn on_exit(&mut self, _: &mut Ui) -> bool {
		!self.block_close_button
	}

	fn app(&mut self, ui: &mut Ui) {
		let scale_factor = 1.5;
		ui.scale_factor(scale_factor);
		ui.style_mut().space = 16.0;
		let card_width = 600.0;
		let card_height = 800.0;

		let card_position = (ui.window_area().width_and_height() / scale_factor - Vec2::new(card_width, card_height)) / 2.0;
		ui.message_provider("msg_provider", |ui, msg| {
			ui.show(&mut Card::new("main")
				.set_rounding(Vec2::same(16.0))
				.set_position(card_position)
				.set_height(card_height)
				.set_width(card_width)
				.set_scrollable([true; 2]),
				|ui, _| {
				ui.label(format!("{:.2}fps", 1.0 / ui.delay().as_seconds_f32()));
				// ui.empty(Vec2::y(16.0));
				ui.add(Label::new("Hello Nablo!").set_bold(true).set_scale(Vec2::same(2.0)));
				ui.divide_line();
				ui.slider(0.0..=100.0, &mut self.some_value, "some_value");
				ui.dragable_value(&mut self.some_value);
				ui.horizental(|ui| {
					if ui.add(SelectableValue::new(self.status == Status::Error, "Error").status(Status::Error)).is_clicked() {
						self.status = Status::Error;
					};
					if ui.add(SelectableValue::new(self.status == Status::Info, "Info").status(Status::Info)).is_clicked() {
						self.status = Status::Info;
					};
					if ui.add(SelectableValue::new(self.status == Status::Warning, "Warning").status(Status::Warning)).is_clicked() {
						self.status = Status::Warning;
					};
					if ui.add(SelectableValue::new(self.status == Status::Success, "Success").status(Status::Success)).is_clicked() {
						self.status = Status::Success;
					};
					if ui.add(SelectableValue::new(self.status == Status::Default, "Default").status(Status::Default)).is_clicked() {
						self.status = Status::Default;
					};
				});
				ui.single_input(&mut self.some_text);
				ui.horizental(|ui| {
					ui.switch(&mut self.block_close_button, "block close button");
					if ui.button("+").is_clicked() {
						self.some_value += 1.0;
					};
					if ui.button("-").is_clicked() {
						self.some_value -= 1.0;
					};
				});
				ui.progress_bar(self.some_value / 100.0, true, self.status.clone());
				ui.tooltip("ok", format!("{:.0}%", self.some_value), |ui, _| {
					ui.progress_bar(self.some_value / 100.0, false, self.status.clone());
				});
				ui.add(Button::new("stroked").style(ButtonStyle::Stroked));
				ui.add(Button::new("lined").style(ButtonStyle::Lined));
				ui.show(&mut Card::new("widegts preview").set_height(200.0).set_scrollable([true; 2]).set_rounding(Vec2::same(16.0)), |ui, _| {
					ui.collapsing("button gallary", |ui, collapsing| {
						if ui.button("close").is_clicked() {
							collapsing.open(false, ui);
						}
						if ui.add(Button::new("Error").status(Status::Error)).is_clicked() {
							msg.message(Message::from("Error").status(Status::Error), ui);
						}
						if ui.add(Button::new("Info").status(Status::Info)).is_clicked() {
							msg.message(Message::from("Info").status(Status::Info), ui);
						}
						if ui.add(Button::new("Warning").status(Status::Warning)).is_clicked() {
							msg.message(Message::from("Warning").status(Status::Warning), ui);
						}
						if ui.add(Button::new("Success").status(Status::Success)).is_clicked() {
							msg.message(Message::from("Success").status(Status::Success), ui);
						}
						if ui.add(Button::new("Default").status(Status::Default)).is_clicked() {
							msg.message("Default", ui);
						}
					});
				});
				ui.horizental(|ui| {
					if ui.switch(&mut self.is_shader_on, "custom shader").is_clicked() {
						if self.is_shader_on {
							ui.change_current_shader(Some("wow".to_string()));
						}else {
							ui.change_current_shader(None);
						}
					};
				});
				let canvas_size = Vec2::new(ui.window_area().width() - 2.0 * 16.0, 300.0);
				let color = ui.style().card_color.brighter(0.1);
				ui.canvas(canvas_size, |painter| {
					painter.set_clip([Vec2::ZERO, canvas_size].into());
					painter.set_info(1);
					painter.set_color(color);
					painter.rect(canvas_size, Vec2::same(16.0));
				});
				if ui.button("exit").is_clicked() {
					ui.close()
				}
			});
		});
	}
}

fn main() {
	let _ = Manager::new_with_settings(Main::default(), nablo::Settings {
		title: "Shapoist".into(),
		..Default::default()
	}).run();
}