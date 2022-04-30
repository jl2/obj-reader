// main.rs
//
// Copyright (c) 2022 Jeremiah LaRocco <jeremiah_larocco@fastmail.com>
//
// Permission to use, copy, modify, and/or distribute this software for any
// purpose with or without fee is hereby granted, provided that the above
// copyright notice and this permission notice appear in all copies.
//
// THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES
// WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF
// MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR
// ANY SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES
// WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN
// ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF
// OR IN CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.

#[cfg(test)]

extern crate obj_reader;

mod tests {

    #[test]
    fn read_cube() {
        let obj = obj_reader::obj::obj_from_file("tests/models/cube.obj".to_string()).unwrap();
        assert_eq!(obj.file_name, "tests/models/cube.obj".to_string());
        assert_eq!(obj.objects.len(), 1);
        assert_eq!(obj.materials.len(), 1);
        assert_eq!(obj.objects[0].name, "Cube1".to_string());
        assert_eq!(obj.objects[0].vertices.len(), 8 * 3);
        assert_eq!(obj.objects[0].normals.len(), 8 * 3);
        assert_eq!(obj.objects[0].textures.len(), 0);
        assert_eq!(obj.objects[0].parameters.len(), 0);
        assert_eq!(obj.objects[0].groups.len(), 1);
        assert_eq!(obj.objects[0].groups[0].name, "Cube1_default");
        println!("{:?}", obj);
    }

    #[test]
    fn read_cube_and_sphere() {
        let obj = obj_reader::obj::obj_from_file("tests/models/cube_and_sphere.obj".to_string()).unwrap();
        assert_eq!(obj.file_name, "tests/models/cube_and_sphere.obj".to_string());
        assert_eq!(obj.objects.len(), 2);
        assert_eq!(obj.materials.len(), 1);
        assert_eq!(obj.objects[0].name, "sphere2".to_string());
        assert_eq!(obj.objects[1].name, "Cube1".to_string());
        println!("{:?}", obj);
    }

}
