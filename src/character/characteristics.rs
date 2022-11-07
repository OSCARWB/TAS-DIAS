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
		ui.label(Self::calc_modifier(value).to_string());
		ui.end_row();
	}

	pub fn draw(& self,ui: &mut Ui)
	{
		egui::Frame::default().show(ui, |ui|
			{
				ui.group(|ui|{
					
					ui.heading("Characteristics");
					egui::Grid::new("characteristics").show(ui, |ui|
						{
							ui.label("");
							ui.label("Base");
							//ui.label("Temp");
							ui.label("Mod");
							ui.end_row();
							Self::draw_stat(ui, "Strength".to_string(), self.strength);
							Self::draw_stat(ui, "Dexterity".to_string(), self.dexterity);
							Self::draw_stat(ui, "Endurance".to_string(), self.endurance);
							Self::draw_stat(ui, "Intelligence".to_string(), self.intelligence);
							Self::draw_stat(ui, "Education".to_string(), self.education);
							Self::draw_stat(ui, "Socual Standing".to_string(), self.social_standing);
							Self::draw_stat(ui, "Psionics".to_string(), self.psionics);
						});
				});
			});
	}
}