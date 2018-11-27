use<../f3-eva/printed/mpu9250.scad>;

$fn = 30;
mpu9250_dim = [26.1, 15.6, 0] + [2.6, 2.6, 2.6];
mpu9250_pos = -mpu9250_dim/2 + [0, -5, 5 - 1.3];

difference() {
    cube(100, center=true);
    translate([0, -25, 0]) {
        cube(75, center=true);
    }
        translate([0, 25, 0]) {
        cube(75, center=true);
    }
    translate([25, 0, 0]) {
        cube(75, center=true);
    }
    translate([-25, 0, 0]) {
        cube(75, center=true);
    }
    translate([0, 0, 25]) {
        cube(75, center=true);
    }
    translate([0, 0, -25]) {
        cube(75, center=true);
    }
}

intersection() {
    union() {
        rotate([0, 30, 45]) {
            translate(mpu9250_pos) {
                mpu9250();
            }
            cube([250, 10, 10], center=true);
        }
    }
    cube(100, center=true);
}