use crate::character::Character;

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct TemplateApp {
	// // Example stuff:
	// label: String,

	// // this how you opt-out of serialization of a member
	// //#[serde(skip)]
	// value: f32,
	character: Character
}

impl Default for TemplateApp {
	fn default() -> Self {
		Self {
			// Example stuff:
			// label: "Hello World!".to_owned(),
			// value: 2.7,
			character: Character::default(),
		}
	}
}

impl TemplateApp {
	/// Called once before the first frame.
	pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
		// This is also where you can customized the look at feel of egui using
		// `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

		// Load previous app state (if any).
		// Note that you must enable the `persistence` feature for this to work.
		if let Some(storage) = cc.storage {
			return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
		}

		Default::default()
	}
}

impl eframe::App for TemplateApp {
	/// Called by the frame work to save state before shutdown.
	fn save(&mut self, storage: &mut dyn eframe::Storage) {
		eframe::set_value(storage, eframe::APP_KEY, self);
	}

	fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
		let Self {character} = self;
		egui::CentralPanel::default().show(&ctx, |ui|{
			ui.horizontal(|ui|{
				ui.label("Name :");
				ui.label("Hirari of Ondrata Minor");
				ui.separator();
				ui.label("Age :");
				ui.label("44");
				ui.separator();
				ui.label("Terms :");
				ui.label("2");
				ui.separator();
				ui.label("Race :");
				ui.label("Human");
				ui.separator();
				ui.label("Homeworld :");
				ui.label("Ξ Ondratae Minoris");
			});
			ui.separator();
			character.characteristics.draw(ui);
		});
	}
}
