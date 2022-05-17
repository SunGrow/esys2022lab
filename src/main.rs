#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::egui;

const REASON_NAMES: [&str; 61] = [
    "ранение гр.клетки, живота, спины",
    "ранение головы, шеи (в сознании)",
    "огнестрельное ранение",
    "обширная термическая травма",
    "травматич. ампутация конечностей",
    "множественная, сочетанная травма",
    "задыхается (аллергия)",
    "судороги (высокая температура)",
	"судороги (отравление)",
	"рвота с кровью",
	"роды, кровотечение",
	"роды, отошли воды + кровотечение",
	"болит живот + рвота",
	"без сознания (бронхиальная астма)",
	"без сознания (подавился)",
    "без сознания (угорел)",
	"без сознания (диабет)",
	"без сознания (передозировка нарко)",	
    "без сознания (травма)",
	"без сознания (кардио больной)",
	"без сознания (высокая температура)",
	"без сознания (припадок)",
	"без сознания (отравился)",
	"без сознания (утопление)",
	"без сознания (причина неизвестна)",
	"без сознания (острое кровотечение)",
	"без сознания (анафилактический шок)",
	"без сознания (повесился)",
	"ДТП (1-2, взрослые)",
	"ДТП (1-2, есть дети)",	  	 
	"ДТП (более 2-х, взрослые)", 	  	 
	"ДТП (более 2-х, есть дети)",	  	 
	"наезд на пешехода",
	"пожар (1-2, взрослые)",
	"пожар (1-2, есть дети)",
	"пожар (более 2, взросл.)",
	"пожар (более 2, дети)",
	"взрыв, ЧС (1-2, взрослые)", 	  	 
	"взрыв, ЧС (1-2, есть дети)", 	  	 
	"взрыв, ЧС (более 2, взросл)", 	  	 
	"взрыв, ЧС (более 2, дети)",
	"дежурство при угрозе теракта",
	"падение с высоты (в сознании)",
    "ушиб, перелом конечностей",
    
    "аритмия",
    "парализовало",
    "ДТП (дежурство)",
    "пожар (дежурство)",
    "взрыв, ЧС (дежурство)",

    "передозировка наркотиков",
    "травма глаза",
    "травма уха",
    "повышенное АД",
    "болит живот",
    "роды",
    "роды, отошли воды",
    
    "низкое давление, головокружение",
    "сыпь",
    "без сознания (онко)",
    "вызов на себя фельдшерской бригады",

    "инородное тело носа",
];

const REASON_IMPORTANCE: [&u32; 61] = [
	&0,
	&0,
	&0,
	&0,
	&0,
	&0,
	&0,
	&0,
	&0,
	&0,
	&0,
	&0,
	&0,
	&0,
	&0,
	&0,
	&0,
	&0,
	&0,
	&0,
	&0,
	&0,
	&0,
	&0,
	&0,
	&0,
	&0,
	&0,
	&0,
	&0,
	&0,
	&0,
	&0,
	&0,
	&0,
	&0,
	&0,
	&0,
	&0,
	&0,
	&0,
	&0,
	&0,
	&0,
	&1,
	&1,
	&1,
	&1,
	&1,
	&2,
	&2,
	&2,
	&2,
	&2,
	&2,
	&2,
	&3,
	&3,
	&3,
	&3,
	&4,
];

pub fn main(){
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Lab 4",
        options,
        Box::new(|_cc| Box::new(LabApp::default())),
    );
}

struct CallResason {
    name: String,
    priority: u32,
}

struct LabApp {
    reason: CallResason,
}

impl Default for LabApp {
    fn default() -> Self {
        let init_reason = CallResason{
            name: String::from(""),
            priority: 0,
        };
        Self {
            reason: init_reason
        }
    }
}

impl eframe::App for LabApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Оценка категории срочности вызова");
            ui.horizontal(|ui| {
                ui.label("Повод для вызова: ");
                ui.add(egui::TextEdit::singleline(&mut self.reason.name)
                       .hint_text("Поле для повода вызова")
                       );
            ui.add(egui::Label::new(""));
            });
        });
    }
}
