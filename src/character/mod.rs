use egui::Ui;

mod career;
mod characteristics;

use self::characteristics::Characteristics;
use self::career::Career;

#[derive(serde::Deserialize, serde::Serialize)]
pub struct Character
{
	pub name: String,
	pub age: String,
	pub terms: u8,
	pub race: String,
	pub homeworld: String,
	pub characteristics: Characteristics,
	pub career_history: std::vec::Vec<Career>,
}

impl Default for Character
{
	fn default() -> Self {
		Self {
			name: "Hirari of Ondrata Minor".to_owned(),
			age: "N/A".to_owned(),
			terms: 0,
			race: "Human".to_owned(),
			homeworld: "Ξ Ondratae Minoris".to_owned(),
			characteristics: Characteristics::default(),
			career_history: vec![Career::default()], 
		}
	}
}

impl Character
{
	pub fn draw_basics(&self,ui:&mut Ui)
	{
		ui.horizontal_wrapped(|ui|{
			egui::global_dark_light_mode_switch(ui);
			ui.separator();
			//ui.group(|ui|{
				ui.label("Name :");
				ui.strong(self.name.clone());
			//});
			ui.separator();
			//ui.group(|ui|{
				ui.label("Age :");
				ui.strong(self.age.clone());
			//});
			ui.separator();
			//ui.group(|ui|{
				ui.label("Terms :");
				ui.strong(self.age.to_string());
			//});
			ui.separator();
			//ui.group(|ui|{
				ui.label("Race :");
				ui.strong(self.race.clone());
			//});
			ui.separator();
			//ui.group(|ui|{
				ui.label("Homeworld :");
				ui.strong(self.homeworld.clone());
			//});
		});
	}
}