use egui::Ui;

use self::characteristics::Characteristics;
mod characteristics;

#[derive(serde::Deserialize, serde::Serialize)]
pub struct Character
{
	pub name: String,
	pub age: String,
	pub terms: u8,
	pub race: String,
	pub homeworld: String,
	pub characteristics: Characteristics,
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
		}
	}
}

impl Character
{
	pub fn draw_basics(&self,ui:&mut Ui)
	{
		ui.horizontal(|ui|{
			egui::global_dark_light_mode_switch(ui);
			ui.separator();
			ui.label("Name :");
			ui.label(self.name.clone());
			ui.separator();
			ui.label("Age :");
			ui.label(self.age.clone());
			ui.separator();
			ui.label("Terms :");
			ui.label(self.age.to_string());
			ui.separator();
			ui.label("Race :");
			ui.label(self.race.clone());
			ui.separator();
			ui.label("Homeworld :");
			ui.label(self.homeworld.clone());
		});
	}
}