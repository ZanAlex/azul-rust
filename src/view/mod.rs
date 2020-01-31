use crate::data::{ Bag, Coaster, PlayerBoard, Line, Table, Tile };

use std::io::Write;

use crate::termcolor::WriteColor;

pub struct View {
    stdout: termcolor::StandardStream,

    coaster_colorspec: termcolor::ColorSpec,
    tile_colorspec: Vec<(Tile, termcolor::ColorSpec)>,

    error_colorspec: termcolor::ColorSpec,
}

impl View {
    pub fn new() -> View {
        let mut result = View {
            stdout: termcolor::StandardStream::stdout(termcolor::ColorChoice::Always),
            coaster_colorspec: termcolor::ColorSpec::new(),
            tile_colorspec: Vec::new(),
            error_colorspec: termcolor::ColorSpec::new(),
        };
    
        result.error_colorspec
            .set_bold(true)
            .set_bg(Some(termcolor::Color::Magenta));

        result.coaster_colorspec
            .set_bold(true);

        let create_spec = |color: termcolor::Color| -> termcolor::ColorSpec {  
            let mut spec = termcolor::ColorSpec::new();
            spec.set_fg(Some(color));
            return spec;
        };

        result.tile_colorspec.push((Tile::Blue, create_spec(termcolor::Color::Blue)));
        result.tile_colorspec.push((Tile::Dark, create_spec(termcolor::Color::Magenta)));
        result.tile_colorspec.push((Tile::Red, create_spec(termcolor::Color::Red)));
        result.tile_colorspec.push((Tile::Turquoise, create_spec(termcolor::Color::Cyan)));
        result.tile_colorspec.push((Tile::Yellow, create_spec(termcolor::Color::Yellow)));

        return result;
    }

    pub fn present_table(&mut self, table: &Table) {
        let mut index = 1;
        for coaster in table.coasters.iter() {
            if index > 1 {
                write!(&mut self.stdout, " / ");
            }

            self.stdout.set_color(&self.coaster_colorspec);
            write!(&mut self.stdout, "Coaster {}: ", index);
            self.stdout.reset();

            index = index + 1;

            for tile in coaster.tiles.iter() {
                self.present_tile(&tile);
            }
        }

        self.new_line();
    }

    pub fn present_bag(&mut self, bag: &Bag) {
        write!(&mut self.stdout, "Bag: ");

        for tile in bag.reserve.iter() {
            self.present_tile(&tile);
        }

        self.new_line();
    }

    pub fn present_playerboard(&mut self, playerboard: &PlayerBoard) {
        writeln!(&mut self.stdout, "- Player {} -", playerboard.number);

        for line_index in 0..playerboard.lines.len() {
            let line = &playerboard.lines[line_index];
            let empties = line.capacity() - line.tiles.len();

            for tile in line.tiles.iter() {
                self.present_tile(tile);
            }

            for _ in 0..empties {
                write!(&mut self.stdout, "X");
            }

            let distance_to_wall = 5 - line.capacity() + 2;
            for _ in 0..distance_to_wall {
                write!(&mut self.stdout, " ");
            }

            for column_index in 0..5 {
                match playerboard.wall.get_cell(line_index, column_index) {
                    Some(tile) => self.present_tile(&tile),
                    None => write!(&mut self.stdout, ".").unwrap(),
                };
            }

            self.new_line();
        }

        for option_tile in playerboard.garbage.iter() {
            match option_tile {
                Some(tile) => self.present_tile(&tile),
                None => write!(&mut self.stdout, " ").unwrap(),
            }
        }

        self.new_line();
    }

    fn present_tile(&mut self, tile: &Tile) {
        let color_spec = match self.tile_colorspec.iter().find(|(t, _)| t == tile) {
            Some((_, cs)) => &cs,
            None => &self.error_colorspec,
        };

        self.stdout.set_color(color_spec);
        write!(&mut self.stdout, "T");
        self.stdout.reset();
    }

    fn new_line(&mut self) {
        writeln!(&mut self.stdout, "");
    }
}