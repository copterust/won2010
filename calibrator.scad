use<../f3-eva/printed/mpu9250.scad>;

$fn = 30;
cube_size = 50;

mpu9250_dim = [26.1, 15.6, 0] + [2.6, 2.6, 2.6];
mpu9250_pos = -mpu9250_dim/2 + [0, -cube_size/20 + 2.0, cube_size/20 - 1.3];

difference() {
    cube(cube_size, center=true);
    translate([0, -cube_size / 4, 0]) {
        cube(cube_size - cube_size / 4, center=true);
    }
        translate([0, cube_size / 4, 0]) {
        cube(cube_size - cube_size / 4, center=true);
    }
    translate([cube_size / 4, 0, 0]) {
        cube(cube_size - cube_size / 4, center=true);
    }
    translate([-cube_size / 4, 0, 0]) {
        cube(cube_size - cube_size / 4, center=true);
    }
    translate([0, 0, cube_size / 4]) {
        cube(cube_size - cube_size / 4, center=true);
    }
    translate([0, 0, -cube_size / 4]) {
        cube(cube_size - cube_size / 4, center=true);
    }
}

center_len = sqrt(pow(sqrt(pow(2 * cube_size, 2)), 2) + pow(cube_size, 2));
intersection() {
    union() {
        rotate([0, 30, 45]) {
            translate(mpu9250_pos) {
                mpu9250();
            }
            cube([center_len, cube_size / 10, cube_size / 10], center=true);
        }
    }
    cube(cube_size, center=true);
}