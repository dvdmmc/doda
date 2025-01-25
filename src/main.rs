mod network;
mod data;
mod crypto;
mod search;
mod identity;

use clap::Parser;
use eframe::egui;
use identity::IdentityManager;
use std::io::{self, Write};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
	/// Run in CLI mode instead of GUI
	#[arg(short, long)]
	cli: bool,
}

struct DodaApp {
	identity_manager: IdentityManager,
	name_input: String,
	recovery_key_input: String,
	status_message: String,
}

impl DodaApp {
	fn new() -> Self {
		Self {
			identity_manager: IdentityManager::new(),
			name_input: String::new(),
			recovery_key_input: String::new(),
			status_message: String::new(),
		}
	}
}

impl eframe::App for DodaApp {
	fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
		egui::CentralPanel::default().show(ctx, |ui| {
			ui.heading("P2P OSINT Network Node");
			
			if !self.status_message.is_empty() {
				ui.label(&self.status_message);
				ui.separator();
			}

			ui.horizontal(|ui| {
				ui.label("Name (optional):");
				ui.text_edit_singleline(&mut self.name_input);
				if ui.button("Generate New Identity").clicked() {
					let name = if self.name_input.is_empty() { 
						None 
					} else { 
						Some(self.name_input.clone()) 
					};
					match self.identity_manager.generate_new_identity(name) {
						Ok(_) => self.status_message = "New identity generated successfully!".to_string(),
						Err(e) => self.status_message = format!("Error generating identity: {}", e),
					}
				}
			});

			ui.separator();

			if ui.button("Export Recovery Key").clicked() {
				match self.identity_manager.export_recovery_key() {
					Ok(key) => self.status_message = format!("Recovery key: {}", key),
					Err(e) => self.status_message = format!("Error exporting recovery key: {}", e),
				}
			}

			ui.separator();

			ui.horizontal(|ui| {
				ui.label("Recovery Key:");
				ui.text_edit_singleline(&mut self.recovery_key_input);
				if ui.button("Import Identity").clicked() {
					match self.identity_manager.import_identity(&self.recovery_key_input) {
						Ok(_) => self.status_message = "Identity imported successfully!".to_string(),
						Err(e) => self.status_message = format!("Error importing identity: {}", e),
					}
				}
			});
		});
	}
}

async fn run_cli_mode() {
	println!("P2P OSINT Network Node");
	let mut identity_manager = IdentityManager::new();

	loop {
		println!("\nOptions:");
		println!("1. Generate new identity");
		println!("2. Export recovery key");
		println!("3. Import identity");
		println!("4. Exit");
		print!("\nChoice: ");
		io::stdout().flush().unwrap();

		let mut input = String::new();
		io::stdin().read_line(&mut input).unwrap();

		match input.trim() {
			"1" => {
				print!("Enter name (optional): ");
				io::stdout().flush().unwrap();
				let mut name = String::new();
				io::stdin().read_line(&mut name).unwrap();
				let name = if name.trim().is_empty() { None } else { Some(name.trim().to_string()) };
				
				match identity_manager.generate_new_identity(name) {
					Ok(_) => println!("New identity generated successfully!"),
					Err(e) => println!("Error generating identity: {}", e),
				}
			}
			"2" => {
				match identity_manager.export_recovery_key() {
					Ok(key) => println!("Recovery key: {}", key),
					Err(e) => println!("Error exporting recovery key: {}", e),
				}
			}
			"3" => {
				print!("Enter recovery key: ");
				io::stdout().flush().unwrap();
				let mut key = String::new();
				io::stdin().read_line(&mut key).unwrap();
				
				match identity_manager.import_identity(key.trim()) {
					Ok(_) => println!("Identity imported successfully!"),
					Err(e) => println!("Error importing identity: {}", e),
				}
			}
			"4" => break,
			_ => println!("Invalid option"),
		}
	}
}

#[tokio::main]
async fn main() {
	let args = Args::parse();

	if args.cli {
		run_cli_mode().await;
	} else {
		let options = eframe::NativeOptions {
			initial_window_size: Some(egui::vec2(480.0, 400.0)),
			..Default::default()
		};

		eframe::run_native(
			"DODA",
			options,
			Box::new(|_cc| Box::new(DodaApp::new())),
		).unwrap();
	}
}