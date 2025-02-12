#[derive(Debug)]
struct File {
    name: String
}

#[derive(Debug)]
struct Folder {
    name: String,
    contents: Vec<File>
}

impl Folder {
    fn new(name:String) -> Folder {
        Folder {
            name,
            contents: Vec::new()
        }
    }

    fn create_file(&mut self, name: String) {
        let file = File {
            name
        };
        self.contents.push(file)
    }


    fn delete_file(&mut self, index: usize) -> File {
        let deleted_file = self.contents.remove(index);
        return deleted_file;
    }

    fn get_file(&self, index: usize) -> Option<&File> {
        let file = self.contents.get(index);
        return file;
    }

}


fn main() {

let mut test_folder = Folder::new(String::from("test_folder"));
println!("{test_folder:?}");

test_folder.create_file("file-1.txt".to_string());
test_folder.create_file("file-2.txt".to_string());

println!("{:#?}", test_folder);

let deleted_file = test_folder.delete_file(0);
println!("deleted file name is: {:?}", deleted_file);

println!("{:#?}", test_folder);

let index: usize = 0;
let get_file_option = test_folder.get_file(index);
    match get_file_option {
        
        Some(file_ref) => println!("file found at {index} location: {:#?}", file_ref),
        None => println!("There was no file!")

    }

//vector immutability checks

    let mut cities = vec![
        String::from("Nashville"),
        String::from("Cleveland"),
        String::from("Cincinnati"),
    ];
 
    let target = &mut cities[1];
    let destination = &cities[1];
    println!("{destination}");


}
