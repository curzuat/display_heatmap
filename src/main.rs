extern crate csv;
extern crate image;
extern crate serde;
extern crate lexical;

use std::error::Error;
//use std::io;
use std::process;
use std::env;

use serde::{Deserialize};



//use image::{GenericImage, GenericImageView, ImageBuffer, RgbImage};
use image::{ImageBuffer, RgbImage};

#[derive(Debug, Deserialize)]
struct Record {
    row: u32,
    column: u32,
    color: String,
}



fn example(target_file: &str) -> Result<(), Box<Error>> {
    // Build the CSV reader and iterate over each record.
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(false)
        .delimiter(b',')
        .from_path(target_file)?;
        //.from_path("test.tsv")?;


    let mut img: RgbImage = ImageBuffer::new(0, 0);
    let mut flag = false;

    /*

    {
	    let mut iter = rdr.deserialize();

	    if let Some(result) = iter.next() {
	        let record: Record = result?;

	        rows = record.row;
	        columns = record.column;
	        color = record.color
	        	.split("_")
	        	.map(|c| lexical::parse::<u8, _>(c).unwrap())
	        	.collect();

	        img = ImageBuffer::new(rows + 1, columns + 1);

	        img.put_pixel(rows, columns, image::Rgb([color[0], color[1], color[2]]));




	    } else {
	    	panic!("No fields!")
	    }
	}
	*/
    

    



    for result in rdr.deserialize() {
        // The iterator yields Result<StringRecord, Error>, so we check the
        // error here.
        let record: Record = result?;
        	let rows = record.row;
	        let columns = record.column;
	        let color: Vec<u8> = record.color
	        	.split("_")
	        	.map(|c| lexical::parse::<u8, _>(c).unwrap())
	        	.collect();

	        if flag == false{
	        	flag = true;
	        	img = ImageBuffer::new(rows + 1, columns + 1);
	        	img.put_pixel(rows, columns, image::Rgb([color[0], color[1], color[2]]));
	        } else{
	        	img.put_pixel(rows, columns, image::Rgb([color[0], color[1], color[2]]));

	        }

	        


    }
    

    img.save("heatmap.png").unwrap();
    //println!("{}", _total);
    Ok(())
}

fn main() {

	let args: Vec<String> = env::args().collect();
	let target_file = &args[1];

    if let Err(err) = example(target_file) {
        println!("error running example: {}", err);
        process::exit(1);
    }
}