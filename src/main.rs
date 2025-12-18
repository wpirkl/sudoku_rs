use eframe::egui;

use rand::rngs::StdRng;
use rand::SeedableRng;

use sudoku::sudoku_pencil_notes::{PencilNotes, RandomBit};

use sudoku::sudoku_pencil_notes_fmt::*;

// --------------------------

struct SudokuApp {
    notes: PencilNotes<9, 9>,
    random_bit: RandomBit,
}

impl SudokuApp {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        
        Self {
            notes: PencilNotes::new(),
            random_bit: RandomBit::new(Box::new(StdRng::seed_from_u64(42))),
        }
    }

    fn get_possible_numbers(mask: u32) -> Vec<u8> {
        let mut numbers = Vec::new();
        for i in 0..9 {
            if (mask >> i) & 1 == 1 {
                numbers.push((i + 1) as u8);
            }
        }
        numbers
    }
}

impl eframe::App for SudokuApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Sudoku Pencil Notes");
            ui.label("Click a cell to collapse it to a random valid possibility.");
            ui.add_space(20.0);

            // Define grid styling
            let cell_size = 60.0; // Size of one square cell
            let spacing = 0.0;    // No gap between cells, we draw borders manually
            
            // Create a Grid layout
            egui::Grid::new("sudoku_grid")
                .spacing(egui::vec2(spacing, spacing))
                .show(ui, |ui| {
                    for row in 0..9 {
                        for col in 0..9 {
                            // 1. ALLOCATE THE CELL
                            // We reserve space for the cell first to get its Rectangle (rect)
                            let (rect, response) = ui.allocate_exact_size(
                                egui::vec2(cell_size, cell_size), 
                                egui::Sense::click()
                            );

                            // 2. DATA LOGIC
                            let mask = self.notes.possibilities[row][col];
                            let possibilities = Self::get_possible_numbers(mask);
                            let is_solved = possibilities.len() == 1;

                            // 3. INTERACTION
                            // If clicked, pick a random number from the possibilities
                            if response.clicked() && !is_solved {
                                println!("Clicked!");
                                if let Some(number) = self.random_bit.select_random_bit(mask) {
                                    let number = number + 1;
                                    self.notes.set_possibility(row, col, number);
                                    self.notes.eliminate_possibility(row, col, number);

                                    println!("Mask: 0b{:09b}, Selected {}, Pencil Notes are now:\n{}", mask, number, self.notes);
                                }
                            }

                            // 4. PAINTING
                            // We use the painter to draw directly onto the allocated rect
                            let painter = ui.painter();

                            // A. Draw background and basic cell border
                            let bg_color = if response.hovered() && !is_solved {
                                egui::Color32::from_gray(40) // Highlight on hover
                            } else {
                                egui::Color32::from_gray(20)
                            };
                            painter.rect_filled(rect, 0.0, bg_color);
                            painter.rect_stroke(rect, 0.0, egui::Stroke::new(1.0, egui::Color32::from_gray(60)), egui::StrokeKind::Outside);

                            // B. Draw Content
                            if is_solved {
                                // DRAW BIG NUMBER
                                painter.text(
                                    rect.center(),
                                    egui::Align2::CENTER_CENTER,
                                    format!("{}", possibilities[0]),
                                    egui::FontId::proportional(32.0),
                                    egui::Color32::WHITE,
                                );
                            } else {
                                // DRAW PENCIL GRID (3x3)
                                // We divide the cell rect into 3x3 sub-rects
                                let sub_w = rect.width() / 3.0;
                                let sub_h = rect.height() / 3.0;

                                for &num in &possibilities {
                                    // Calculate x,y index (0-2) for the number (1-9)
                                    // 1->(0,0), 2->(1,0), 3->(2,0) ... 9->(2,2)
                                    let idx = num - 1;
                                    let sub_x = idx % 3;
                                    let sub_y = idx / 3;

                                    let center_x = rect.min.x + (sub_x as f32 * sub_w) + (sub_w / 2.0);
                                    let center_y = rect.min.y + (sub_y as f32 * sub_h) + (sub_h / 2.0);
                                    let pos = egui::pos2(center_x, center_y);

                                    painter.text(
                                        pos,
                                        egui::Align2::CENTER_CENTER,
                                        format!("{}", num),
                                        egui::FontId::proportional(12.0),
                                        egui::Color32::from_gray(170),
                                    );
                                }
                            }

                            // C. Draw THICK Borders for 3x3 Blocks
                            // We draw a thicker line on the right/bottom if it's a block edge
                            let stroke_thick = egui::Stroke::new(3.0, egui::Color32::WHITE);
                            
                            // Right border for columns 2 and 5
                            if col == 2 || col == 5 {
                                painter.line_segment(
                                    [rect.right_top(), rect.right_bottom()], 
                                    stroke_thick
                                );
                            }
                            // Bottom border for rows 2 and 5
                            if row == 2 || row == 5 {
                                painter.line_segment(
                                    [rect.left_bottom(), rect.right_bottom()], 
                                    stroke_thick
                                );
                            }
                        }
                        ui.end_row();
                    }
                });
        });
    }
}

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([600.0, 700.0]),
        ..Default::default()
    };
    
    eframe::run_native(
        "Sudoku Pencil Notes",
        options,
        Box::new(|cc| Ok(Box::new(SudokuApp::new(cc)))),
    )
}