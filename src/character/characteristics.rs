use egui::Ui;

#[derive(serde::Deserialize, serde::Serialize, Copy, Clone)]
pub struct Characteristics
{
	pub strength: i64,
	pub dexterity: i64,
	pub endurance: i64,
	pub intelligence: i64,
	pub education: i64,
	pub social_standing: i64,
	pub psionics: i64,
}

impl Default for Characteristics
{
	fn default() -> Self {
		Self{
			strength: 7,
			dexterity: 7,
			endurance: 7,
			intelligence: 7,
			education: 7,
			social_standing: 7,
			psionics: 0,
		}
	}
}

impl Characteristics
{
	pub fn calc_modifier(stat:i64)->i64
	{
		if stat!=0
		{
			return (stat/3)-2;
		}
		return -3;
	}

	fn draw_stat(ui: &mut Ui, name:String, value:i64)
	{
		ui.label(name);
		ui.label(value.to_string());
		ui.end_row();
	}

	pub fn draw(& self,ui: &mut Ui)
	{
		egui::Frame::default().show(ui, |ui|
			{
				ui.label("Characteristics");
				egui::Grid::new("characteristics").show(ui, |ui|
				{
					ui.label("");
					ui.label("Base");
					//ui.label("Temp");
					//ui.label("Mod");
					ui.end_row();
					Self::draw_stat(ui, "Strength".to_string(), self.strength);
					ui.label("Dexterity");
					ui.label("7");
					ui.label("7");
					ui.label("7");
					ui.end_row();
					ui.label("Endurance");
					ui.label("7");
					ui.label("7");
					ui.label("7");
					ui.end_row();
					ui.label("Intelligence");
					ui.label("7");
					ui.label("7");
					ui.label("7");
					ui.end_row();
					ui.label("Education");
					ui.label("7");
					ui.label("7");
					ui.label("7");
					ui.end_row();
					ui.label("Social Standing");
					ui.label("7");
					ui.label("7");
					ui.label("7");
					ui.end_row();
					ui.label("Psionics");
					ui.label("7");
					ui.label("7");
					ui.label("7");
				});
			});
	}
}