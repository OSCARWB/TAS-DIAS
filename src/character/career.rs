use egui::Ui;

#[derive(serde::Deserialize, serde::Serialize, Clone)]
pub struct Career
{
	pub career: String,
	pub branch: String,
	pub events: String,
	pub rank:   String,
	pub title:  String,
}

impl Default for Career
{
	fn default() -> Self 
	{
		Self {
			career: "".to_string(),
			branch: "".to_string(),
			events: "".to_string(),
			rank:   "".to_string(),
			title:  "".to_string(),
		}
	}
}

impl Career
{
	pub fn draw_careers(careers: std::vec::Vec<Career>)
	{

	}

	fn draw(&self,ui:&mut Ui)
	{

	}
}