$fn = 30;
r_hole = 1.3;
r_snap = 1.6;
d_hole = 24;

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

module clamp() {
    difference() {
        union() {
            cylinder(5, r_hole, r_hole, center=true);
            translate([0, 0, 1]) {
                cylinder(3, r_snap, r_hole);
            }
        }
        translate([0, 0, 3]) {
            scale([1, 5, 8]) {
                cube(1.0, center=true);
            }
        }
    }
}

intersection() {
    union() {
    rotate([0, 0, 45]) {
        translate([d_hole/2,0,5]) {
            clamp();
        }
        translate([-d_hole/2,0,5]) {
            clamp();
        }
        scale([250, 10, 10]) {
            cube(1, center=true);
        }
    }
}
    cube(100, center=true);
}