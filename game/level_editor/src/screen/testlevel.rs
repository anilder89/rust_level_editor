use macroquad::prelude::Color;

use super::{*, editor::{Node, Line}, basics::Player};

pub fn return_test_level() -> LevelState {
    LevelState {
        nodes: vec![
            Node {
                position: Point { x: 64.0, y: -41.0 },
                radius: 5.0,
                selected: false,
            },
            Node {
                position: Point { x: 83.0, y: -48.0 },
                radius: 5.0,
                selected: false,
            },
            Node {
                position: Point { x: 72.0, y: 30.0 },
                radius: 5.0,
                selected: false,
            },
            Node {
                position: Point { x: 96.0, y: 35.0 },
                radius: 5.0,
                selected: false,
            },
            Node {
                position: Point { x: 38.0, y: -54.0 },
                radius: 5.0,
                selected: false,
            },
            Node {
                position: Point { x: 71.0, y: -73.0 },
                radius: 5.0,
                selected: false,
            },
            Node {
                position: Point { x: 38.0, y: 36.0 },
                radius: 5.0,
                selected: false,
            },
            Node {
                position: Point { x: 62.0, y: 54.0 },
                radius: 5.0,
                selected: false,
            },
            Node {
                position: Point { x: 94.0, y: 59.0 },
                radius: 5.0,
                selected: false,
            },
            Node {
                position: Point { x: 136.0, y: 32.0 },
                radius: 5.0,
                selected: false,
            },
            Node {
                position: Point { x: 124.0, y: -54.0 },
                radius: 5.0,
                selected: false,
            },
            Node {
                position: Point { x: 113.0, y: -96.0 },
                radius: 5.0,
                selected: false,
            },
            Node {
                position: Point { x: 132.0, y: 70.0 },
                radius: 5.0,
                selected: false,
            },
            Node {
                position: Point { x: 186.0, y: 116.0 },
                radius: 5.0,
                selected: false,
            },
            Node {
                position: Point { x: 243.0, y: 174.0 },
                radius: 5.0,
                selected: false,
            },
            Node {
                position: Point {
                    x: -174.0,
                    y: -183.0,
                },
                radius: 5.0,
                selected: false,
            },
            Node {
                position: Point {
                    x: -160.0,
                    y: 167.0,
                },
                radius: 5.0,
                selected: false,
            },
            Node {
                position: Point {
                    x: 770.34155,
                    y: -4.4664307,
                },
                radius: 5.0,
                selected: false,
            },
            Node {
                position: Point {
                    x: 700.11694,
                    y: 98.954285,
                },
                radius: 5.0,
                selected: false,
            },
            Node {
                position: Point {
                    x: 582.11694,
                    y: 164.95428,
                },
                radius: 5.0,
                selected: false,
            },
            Node {
                position: Point {
                    x: 418.11694,
                    y: 191.95428,
                },
                radius: 5.0,
                selected: false,
            },
            Node {
                position: Point {
                    x: 182.97873,
                    y: -125.69577,
                },
                radius: 5.0,
                selected: false,
            },
            Node {
                position: Point {
                    x: 238.97873,
                    y: -182.69577,
                },
                radius: 5.0,
                selected: false,
            },
            Node {
                position: Point {
                    x: 388.3078,
                    y: -229.86938,
                },
                radius: 5.0,
                selected: false,
            },
            Node {
                position: Point {
                    x: 570.3078,
                    y: -216.86938,
                },
                radius: 5.0,
                selected: false,
            },
            Node {
                position: Point {
                    x: 767.3078,
                    y: -106.869385,
                },
                radius: 5.0,
                selected: false,
            },
            Node {
                position: Point {
                    x: 668.67444,
                    y: -158.14188,
                },
                radius: 5.0,
                selected: false,
            },
            Node {
                position: Point {
                    x: 904.5657,
                    y: -117.4527,
                },
                radius: 5.0,
                selected: false,
            },
            Node {
                position: Point {
                    x: 906.5657,
                    y: -5.4526978,
                },
                radius: 5.0,
                selected: false,
            },
            Node {
                position: Point {
                    x: 1084.5916,
                    y: -124.59296,
                },
                radius: 5.0,
                selected: false,
            },
            Node {
                position: Point {
                    x: 1084.5916,
                    y: 2.4070435,
                },
                radius: 5.0,
                selected: false,
            },
            Node {
                position: Point {
                    x: 1222.31,
                    y: 5.178833,
                },
                radius: 5.0,
                selected: false,
            },
            Node {
                position: Point {
                    x: 1215.31,
                    y: -136.8212,
                },
                radius: 5.0,
                selected: false,
            },
            Node {
                position: Point {
                    x: 1205.0236,
                    y: -324.0785,
                },
                radius: 5.0,
                selected: false,
            },
            Node {
                position: Point {
                    x: 1074.0236,
                    y: -320.0785,
                },
                radius: 5.0,
                selected: false,
            },
            Node {
                position: Point {
                    x: 1062.7882,
                    y: -481.8838,
                },
                radius: 5.0,
                selected: false,
            },
            Node {
                position: Point {
                    x: 1191.7882,
                    y: -493.8838,
                },
                radius: 5.0,
                selected: false,
            },
            Node {
                position: Point {
                    x: 891.9293,
                    y: -469.66968,
                },
                radius: 5.0,
                selected: false,
            },
            Node {
                position: Point {
                    x: 704.9293,
                    y: -472.66968,
                },
                radius: 5.0,
                selected: false,
            },
            Node {
                position: Point {
                    x: 1354.9293,
                    y: -498.66968,
                },
                radius: 5.0,
                selected: false,
            },
            Node {
                position: Point {
                    x: 1479.9293,
                    y: -504.66968,
                },
                radius: 5.0,
                selected: false,
            },
            Node {
                position: Point {
                    x: 1460.1998,
                    y: -851.39795,
                },
                radius: 5.0,
                selected: false,
            },
            Node {
                position: Point {
                    x: 701.1998,
                    y: -845.39795,
                },
                radius: 5.0,
                selected: false,
            },
            Node {
                position: Point {
                    x: 1241.4491,
                    y: 274.9651,
                },
                radius: 5.0,
                selected: false,
            },
            Node {
                position: Point {
                    x: 1268.6156,
                    y: 510.89368,
                },
                radius: 5.0,
                selected: false,
            },
            Node {
                position: Point {
                    x: 1089.6156,
                    y: 508.89368,
                },
                radius: 5.0,
                selected: false,
            },
            Node {
                position: Point {
                    x: 1152.2982,
                    y: -704.1457,
                },
                radius: 5.0,
                selected: false,
            },
            Node {
                position: Point {
                    x: 1294.2982,
                    y: -711.1457,
                },
                radius: 5.0,
                selected: false,
            },
            Node {
                position: Point {
                    x: 784.7189,
                    y: 515.59595,
                },
                radius: 5.0,
                selected: false,
            },
            Node {
                position: Point {
                    x: 794.7189,
                    y: 295.596,
                },
                radius: 5.0,
                selected: false,
            },
            Node {
                position: Point {
                    x: -154.0332,
                    y: 383.48273,
                },
                radius: 5.0,
                selected: false,
            },
            Node {
                position: Point {
                    x: -184.58142,
                    y: -393.98566,
                },
                radius: 5.0,
                selected: false,
            },
            Node {
                position: Point {
                    x: -153.86377,
                    y: -425.58112,
                },
                radius: 5.0,
                selected: false,
            },
            Node {
                position: Point {
                    x: -100.86377,
                    y: -458.58112,
                },
                radius: 5.0,
                selected: false,
            },
            Node {
                position: Point {
                    x: -26.86377,
                    y: -471.58112,
                },
                radius: 5.0,
                selected: false,
            },
            Node {
                position: Point {
                    x: -97.34149,
                    y: 467.99402,
                },
                radius: 5.0,
                selected: false,
            },
            Node {
                position: Point {
                    x: -14.341492,
                    y: 503.99402,
                },
                radius: 5.0,
                selected: false,
            },
            Node {
                position: Point {
                    x: 57.65851,
                    y: 520.994,
                },
                radius: 5.0,
                selected: false,
            },
            Node {
                position: Point {
                    x: 470.9251,
                    y: -474.66516,
                },
                radius: 5.0,
                selected: false,
            },
            Node {
                position: Point {
                    x: 497.9021,
                    y: -840.8209,
                },
                radius: 5.0,
                selected: false,
            },
            Node {
                position: Point {
                    x: 1299.9761,
                    y: -681.2661,
                },
                radius: 5.0,
                selected: false,
            },
            Node {
                position: Point {
                    x: 1158.9761,
                    y: -673.2661,
                },
                radius: 5.0,
                selected: false,
            },
            Node {
                position: Point {
                    x: 809.1261,
                    y: -402.80023,
                },
                radius: 5.0,
                selected: false,
            },
            Node {
                position: Point {
                    x: 809.1261,
                    y: -365.80023,
                },
                radius: 5.0,
                selected: false,
            },
            Node {
                position: Point {
                    x: 846.1261,
                    y: -383.80023,
                },
                radius: 5.0,
                selected: false,
            },
            Node {
                position: Point {
                    x: 847.1261,
                    y: -341.80023,
                },
                radius: 5.0,
                selected: false,
            },
            Node {
                position: Point {
                    x: -128.88544,
                    y: -185.96533,
                },
                radius: 5.0,
                selected: false,
            },
            Node {
                position: Point {
                    x: -122.88544,
                    y: -401.96533,
                },
                radius: 5.0,
                selected: false,
            },
            Node {
                position: Point {
                    x: -23.885437,
                    y: -428.96533,
                },
                radius: 5.0,
                selected: false,
            },
            Node {
                position: Point {
                    x: -209.51257,
                    y: 101.05615,
                },
                radius: 5.0,
                selected: false,
            },
            Node {
                position: Point {
                    x: -169.51257,
                    y: 74.05615,
                },
                radius: 5.0,
                selected: false,
            },
            Node {
                position: Point {
                    x: -116.51257,
                    y: 136.05615,
                },
                radius: 5.0,
                selected: false,
            },
            Node {
                position: Point {
                    x: -77.51257,
                    y: 364.05615,
                },
                radius: 5.0,
                selected: false,
            },
            Node {
                position: Point {
                    x: -18.512573,
                    y: 460.05615,
                },
                radius: 5.0,
                selected: false,
            },
            Node {
                position: Point {
                    x: 779.1825,
                    y: 259.93506,
                },
                radius: 5.0,
                selected: false,
            },
            Node {
                position: Point {
                    x: 815.1825,
                    y: 217.93506,
                },
                radius: 5.0,
                selected: false,
            },
        ],
        lines: vec![
            Line {
                start: Point { x: 38.0, y: -54.0 },
                end: Point { x: 64.0, y: -41.0 },
                color: Color {
                    r: 0.9,
                    g: 0.16,
                    b: 0.22,
                    a: 1.0,
                },
            },
            Line {
                start: Point { x: 64.0, y: -41.0 },
                end: Point { x: 83.0, y: -48.0 },
                color: Color {
                    r: 0.0,
                    g: 0.47,
                    b: 0.95,
                    a: 1.0,
                },
            },
            Line {
                start: Point { x: 38.0, y: -54.0 },
                end: Point { x: 71.0, y: -73.0 },
                color: Color {
                    r: 0.0,
                    g: 0.89,
                    b: 0.19,
                    a: 1.0,
                },
            },
            Line {
                start: Point { x: 38.0, y: 36.0 },
                end: Point { x: 72.0, y: 30.0 },
                color: Color {
                    r: 0.9,
                    g: 0.16,
                    b: 0.22,
                    a: 1.0,
                },
            },
            Line {
                start: Point { x: 38.0, y: 36.0 },
                end: Point { x: 62.0, y: 54.0 },
                color: Color {
                    r: 0.0,
                    g: 0.47,
                    b: 0.95,
                    a: 1.0,
                },
            },
            Line {
                start: Point { x: 72.0, y: 30.0 },
                end: Point { x: 96.0, y: 35.0 },
                color: Color {
                    r: 0.0,
                    g: 0.89,
                    b: 0.19,
                    a: 1.0,
                },
            },
            Line {
                start: Point { x: 62.0, y: 54.0 },
                end: Point { x: 94.0, y: 59.0 },
                color: Color {
                    r: 0.9,
                    g: 0.16,
                    b: 0.22,
                    a: 1.0,
                },
            },
            Line {
                start: Point { x: 71.0, y: -73.0 },
                end: Point { x: 113.0, y: -96.0 },
                color: Color {
                    r: 0.0,
                    g: 0.47,
                    b: 0.95,
                    a: 1.0,
                },
            },
            Line {
                start: Point { x: 113.0, y: -96.0 },
                end: Point {
                    x: 182.97873,
                    y: -125.69577,
                },
                color: Color {
                    r: 0.0,
                    g: 0.89,
                    b: 0.19,
                    a: 1.0,
                },
            },
            Line {
                start: Point {
                    x: 182.97873,
                    y: -125.69577,
                },
                end: Point {
                    x: 238.97873,
                    y: -182.69577,
                },
                color: Color {
                    r: 0.9,
                    g: 0.16,
                    b: 0.22,
                    a: 1.0,
                },
            },
            Line {
                start: Point {
                    x: 238.97873,
                    y: -182.69577,
                },
                end: Point {
                    x: 388.3078,
                    y: -229.86938,
                },
                color: Color {
                    r: 0.0,
                    g: 0.47,
                    b: 0.95,
                    a: 1.0,
                },
            },
            Line {
                start: Point {
                    x: 388.3078,
                    y: -229.86938,
                },
                end: Point {
                    x: 570.3078,
                    y: -216.86938,
                },
                color: Color {
                    r: 0.0,
                    g: 0.89,
                    b: 0.19,
                    a: 1.0,
                },
            },
            Line {
                start: Point {
                    x: 570.3078,
                    y: -216.86938,
                },
                end: Point {
                    x: 668.67444,
                    y: -158.14188,
                },
                color: Color {
                    r: 0.9,
                    g: 0.16,
                    b: 0.22,
                    a: 1.0,
                },
            },
            Line {
                start: Point {
                    x: 668.67444,
                    y: -158.14188,
                },
                end: Point {
                    x: 767.3078,
                    y: -106.869385,
                },
                color: Color {
                    r: 0.0,
                    g: 0.47,
                    b: 0.95,
                    a: 1.0,
                },
            },
            Line {
                start: Point { x: 83.0, y: -48.0 },
                end: Point { x: 124.0, y: -54.0 },
                color: Color {
                    r: 0.0,
                    g: 0.89,
                    b: 0.19,
                    a: 1.0,
                },
            },
            Line {
                start: Point { x: 124.0, y: -54.0 },
                end: Point {
                    x: 182.97873,
                    y: -125.69577,
                },
                color: Color {
                    r: 0.9,
                    g: 0.16,
                    b: 0.22,
                    a: 1.0,
                },
            },
            Line {
                start: Point { x: 96.0, y: 35.0 },
                end: Point { x: 136.0, y: 32.0 },
                color: Color {
                    r: 0.0,
                    g: 0.47,
                    b: 0.95,
                    a: 1.0,
                },
            },
            Line {
                start: Point { x: 94.0, y: 59.0 },
                end: Point { x: 132.0, y: 70.0 },
                color: Color {
                    r: 0.0,
                    g: 0.89,
                    b: 0.19,
                    a: 1.0,
                },
            },
            Line {
                start: Point { x: 132.0, y: 70.0 },
                end: Point { x: 186.0, y: 116.0 },
                color: Color {
                    r: 0.9,
                    g: 0.16,
                    b: 0.22,
                    a: 1.0,
                },
            },
            Line {
                start: Point { x: 186.0, y: 116.0 },
                end: Point { x: 243.0, y: 174.0 },
                color: Color {
                    r: 0.0,
                    g: 0.47,
                    b: 0.95,
                    a: 1.0,
                },
            },
            Line {
                start: Point { x: 243.0, y: 174.0 },
                end: Point {
                    x: 418.11694,
                    y: 191.95428,
                },
                color: Color {
                    r: 0.0,
                    g: 0.89,
                    b: 0.19,
                    a: 1.0,
                },
            },
            Line {
                start: Point {
                    x: 418.11694,
                    y: 191.95428,
                },
                end: Point {
                    x: 582.11694,
                    y: 164.95428,
                },
                color: Color {
                    r: 0.9,
                    g: 0.16,
                    b: 0.22,
                    a: 1.0,
                },
            },
            Line {
                start: Point { x: 136.0, y: 32.0 },
                end: Point { x: 132.0, y: 70.0 },
                color: Color {
                    r: 0.0,
                    g: 0.47,
                    b: 0.95,
                    a: 1.0,
                },
            },
            Line {
                start: Point {
                    x: 582.11694,
                    y: 164.95428,
                },
                end: Point {
                    x: 700.11694,
                    y: 98.954285,
                },
                color: Color {
                    r: 0.0,
                    g: 0.89,
                    b: 0.19,
                    a: 1.0,
                },
            },
            Line {
                start: Point {
                    x: 700.11694,
                    y: 98.954285,
                },
                end: Point {
                    x: 770.34155,
                    y: -4.4664307,
                },
                color: Color {
                    r: 0.9,
                    g: 0.16,
                    b: 0.22,
                    a: 1.0,
                },
            },
            Line {
                start: Point {
                    x: 770.34155,
                    y: -4.4664307,
                },
                end: Point {
                    x: 906.5657,
                    y: -5.4526978,
                },
                color: Color {
                    r: 0.0,
                    g: 0.47,
                    b: 0.95,
                    a: 1.0,
                },
            },
            Line {
                start: Point {
                    x: 906.5657,
                    y: -5.4526978,
                },
                end: Point {
                    x: 1084.5916,
                    y: 2.4070435,
                },
                color: Color {
                    r: 0.0,
                    g: 0.89,
                    b: 0.19,
                    a: 1.0,
                },
            },
            Line {
                start: Point {
                    x: 767.3078,
                    y: -106.869385,
                },
                end: Point {
                    x: 904.5657,
                    y: -117.4527,
                },
                color: Color {
                    r: 0.9,
                    g: 0.16,
                    b: 0.22,
                    a: 1.0,
                },
            },
            Line {
                start: Point {
                    x: 904.5657,
                    y: -117.4527,
                },
                end: Point {
                    x: 1084.5916,
                    y: -124.59296,
                },
                color: Color {
                    r: 0.0,
                    g: 0.47,
                    b: 0.95,
                    a: 1.0,
                },
            },
            Line {
                start: Point {
                    x: 1084.5916,
                    y: -124.59296,
                },
                end: Point {
                    x: 1074.0236,
                    y: -320.0785,
                },
                color: Color {
                    r: 0.0,
                    g: 0.89,
                    b: 0.19,
                    a: 1.0,
                },
            },
            Line {
                start: Point {
                    x: 1074.0236,
                    y: -320.0785,
                },
                end: Point {
                    x: 1062.7882,
                    y: -481.8838,
                },
                color: Color {
                    r: 0.9,
                    g: 0.16,
                    b: 0.22,
                    a: 1.0,
                },
            },
            Line {
                start: Point {
                    x: 1062.7882,
                    y: -481.8838,
                },
                end: Point {
                    x: 891.9293,
                    y: -469.66968,
                },
                color: Color {
                    r: 0.0,
                    g: 0.47,
                    b: 0.95,
                    a: 1.0,
                },
            },
            Line {
                start: Point {
                    x: 891.9293,
                    y: -469.66968,
                },
                end: Point {
                    x: 704.9293,
                    y: -472.66968,
                },
                color: Color {
                    r: 0.0,
                    g: 0.89,
                    b: 0.19,
                    a: 1.0,
                },
            },
            Line {
                start: Point {
                    x: 1191.7882,
                    y: -493.8838,
                },
                end: Point {
                    x: 1205.0236,
                    y: -324.0785,
                },
                color: Color {
                    r: 0.9,
                    g: 0.16,
                    b: 0.22,
                    a: 1.0,
                },
            },
            Line {
                start: Point {
                    x: 1205.0236,
                    y: -324.0785,
                },
                end: Point {
                    x: 1215.31,
                    y: -136.8212,
                },
                color: Color {
                    r: 0.0,
                    g: 0.47,
                    b: 0.95,
                    a: 1.0,
                },
            },
            Line {
                start: Point {
                    x: 1215.31,
                    y: -136.8212,
                },
                end: Point {
                    x: 1222.31,
                    y: 5.178833,
                },
                color: Color {
                    r: 0.0,
                    g: 0.89,
                    b: 0.19,
                    a: 1.0,
                },
            },
            Line {
                start: Point {
                    x: 1222.31,
                    y: 5.178833,
                },
                end: Point {
                    x: 1241.4491,
                    y: 274.9651,
                },
                color: Color {
                    r: 0.9,
                    g: 0.16,
                    b: 0.22,
                    a: 1.0,
                },
            },
            Line {
                start: Point {
                    x: 1241.4491,
                    y: 274.9651,
                },
                end: Point {
                    x: 1268.6156,
                    y: 510.89368,
                },
                color: Color {
                    r: 0.0,
                    g: 0.47,
                    b: 0.95,
                    a: 1.0,
                },
            },
            Line {
                start: Point {
                    x: 1268.6156,
                    y: 510.89368,
                },
                end: Point {
                    x: 1089.6156,
                    y: 508.89368,
                },
                color: Color {
                    r: 0.0,
                    g: 0.89,
                    b: 0.19,
                    a: 1.0,
                },
            },
            Line {
                start: Point {
                    x: 1089.6156,
                    y: 508.89368,
                },
                end: Point {
                    x: 784.7189,
                    y: 515.59595,
                },
                color: Color {
                    r: 0.9,
                    g: 0.16,
                    b: 0.22,
                    a: 1.0,
                },
            },
            Line {
                start: Point {
                    x: 784.7189,
                    y: 515.59595,
                },
                end: Point {
                    x: 57.65851,
                    y: 520.994,
                },
                color: Color {
                    r: 0.0,
                    g: 0.47,
                    b: 0.95,
                    a: 1.0,
                },
            },
            Line {
                start: Point {
                    x: 794.7189,
                    y: 295.596,
                },
                end: Point {
                    x: 1084.5916,
                    y: 2.4070435,
                },
                color: Color {
                    r: 0.0,
                    g: 0.89,
                    b: 0.19,
                    a: 1.0,
                },
            },
            Line {
                start: Point {
                    x: 57.65851,
                    y: 520.994,
                },
                end: Point {
                    x: -14.341492,
                    y: 503.99402,
                },
                color: Color {
                    r: 0.9,
                    g: 0.16,
                    b: 0.22,
                    a: 1.0,
                },
            },
            Line {
                start: Point {
                    x: -14.341492,
                    y: 503.99402,
                },
                end: Point {
                    x: -97.34149,
                    y: 467.99402,
                },
                color: Color {
                    r: 0.0,
                    g: 0.47,
                    b: 0.95,
                    a: 1.0,
                },
            },
            Line {
                start: Point {
                    x: -97.34149,
                    y: 467.99402,
                },
                end: Point {
                    x: -154.0332,
                    y: 383.48273,
                },
                color: Color {
                    r: 0.0,
                    g: 0.89,
                    b: 0.19,
                    a: 1.0,
                },
            },
            Line {
                start: Point {
                    x: -154.0332,
                    y: 383.48273,
                },
                end: Point {
                    x: -160.0,
                    y: 167.0,
                },
                color: Color {
                    r: 0.9,
                    g: 0.16,
                    b: 0.22,
                    a: 1.0,
                },
            },
            Line {
                start: Point {
                    x: -174.0,
                    y: -183.0,
                },
                end: Point {
                    x: -184.58142,
                    y: -393.98566,
                },
                color: Color {
                    r: 0.0,
                    g: 0.47,
                    b: 0.95,
                    a: 1.0,
                },
            },
            Line {
                start: Point {
                    x: -184.58142,
                    y: -393.98566,
                },
                end: Point {
                    x: -153.86377,
                    y: -425.58112,
                },
                color: Color {
                    r: 0.0,
                    g: 0.89,
                    b: 0.19,
                    a: 1.0,
                },
            },
            Line {
                start: Point {
                    x: -153.86377,
                    y: -425.58112,
                },
                end: Point {
                    x: -100.86377,
                    y: -458.58112,
                },
                color: Color {
                    r: 0.9,
                    g: 0.16,
                    b: 0.22,
                    a: 1.0,
                },
            },
            Line {
                start: Point {
                    x: -100.86377,
                    y: -458.58112,
                },
                end: Point {
                    x: -26.86377,
                    y: -471.58112,
                },
                color: Color {
                    r: 0.0,
                    g: 0.47,
                    b: 0.95,
                    a: 1.0,
                },
            },
            Line {
                start: Point {
                    x: -26.86377,
                    y: -471.58112,
                },
                end: Point {
                    x: 470.9251,
                    y: -474.66516,
                },
                color: Color {
                    r: 0.0,
                    g: 0.89,
                    b: 0.19,
                    a: 1.0,
                },
            },
            Line {
                start: Point {
                    x: 704.9293,
                    y: -472.66968,
                },
                end: Point {
                    x: 701.1998,
                    y: -845.39795,
                },
                color: Color {
                    r: 0.9,
                    g: 0.16,
                    b: 0.22,
                    a: 1.0,
                },
            },
            Line {
                start: Point {
                    x: 470.9251,
                    y: -474.66516,
                },
                end: Point {
                    x: 497.9021,
                    y: -840.8209,
                },
                color: Color {
                    r: 0.0,
                    g: 0.47,
                    b: 0.95,
                    a: 1.0,
                },
            },
            Line {
                start: Point {
                    x: 701.1998,
                    y: -845.39795,
                },
                end: Point {
                    x: 1460.1998,
                    y: -851.39795,
                },
                color: Color {
                    r: 0.0,
                    g: 0.89,
                    b: 0.19,
                    a: 1.0,
                },
            },
            Line {
                start: Point {
                    x: 1152.2982,
                    y: -704.1457,
                },
                end: Point {
                    x: 1294.2982,
                    y: -711.1457,
                },
                color: Color {
                    r: 0.9,
                    g: 0.16,
                    b: 0.22,
                    a: 1.0,
                },
            },
            Line {
                start: Point {
                    x: 1158.9761,
                    y: -673.2661,
                },
                end: Point {
                    x: 1152.2982,
                    y: -704.1457,
                },
                color: Color {
                    r: 0.0,
                    g: 0.47,
                    b: 0.95,
                    a: 1.0,
                },
            },
            Line {
                start: Point {
                    x: 1158.9761,
                    y: -673.2661,
                },
                end: Point {
                    x: 1299.9761,
                    y: -681.2661,
                },
                color: Color {
                    r: 0.0,
                    g: 0.89,
                    b: 0.19,
                    a: 1.0,
                },
            },
            Line {
                start: Point {
                    x: 1299.9761,
                    y: -681.2661,
                },
                end: Point {
                    x: 1294.2982,
                    y: -711.1457,
                },
                color: Color {
                    r: 0.9,
                    g: 0.16,
                    b: 0.22,
                    a: 1.0,
                },
            },
            Line {
                start: Point {
                    x: 1191.7882,
                    y: -493.8838,
                },
                end: Point {
                    x: 1354.9293,
                    y: -498.66968,
                },
                color: Color {
                    r: 0.0,
                    g: 0.47,
                    b: 0.95,
                    a: 1.0,
                },
            },
            Line {
                start: Point {
                    x: 1354.9293,
                    y: -498.66968,
                },
                end: Point {
                    x: 1479.9293,
                    y: -504.66968,
                },
                color: Color {
                    r: 0.0,
                    g: 0.89,
                    b: 0.19,
                    a: 1.0,
                },
            },
            Line {
                start: Point {
                    x: 1479.9293,
                    y: -504.66968,
                },
                end: Point {
                    x: 1460.1998,
                    y: -851.39795,
                },
                color: Color {
                    r: 0.9,
                    g: 0.16,
                    b: 0.22,
                    a: 1.0,
                },
            },
            Line {
                start: Point {
                    x: 809.1261,
                    y: -402.80023,
                },
                end: Point {
                    x: 846.1261,
                    y: -383.80023,
                },
                color: Color {
                    r: 0.0,
                    g: 0.47,
                    b: 0.95,
                    a: 1.0,
                },
            },
            Line {
                start: Point {
                    x: 846.1261,
                    y: -383.80023,
                },
                end: Point {
                    x: 847.1261,
                    y: -341.80023,
                },
                color: Color {
                    r: 0.0,
                    g: 0.89,
                    b: 0.19,
                    a: 1.0,
                },
            },
            Line {
                start: Point {
                    x: 847.1261,
                    y: -341.80023,
                },
                end: Point {
                    x: 809.1261,
                    y: -365.80023,
                },
                color: Color {
                    r: 0.9,
                    g: 0.16,
                    b: 0.22,
                    a: 1.0,
                },
            },
            Line {
                start: Point {
                    x: 809.1261,
                    y: -365.80023,
                },
                end: Point {
                    x: 809.1261,
                    y: -402.80023,
                },
                color: Color {
                    r: 0.0,
                    g: 0.47,
                    b: 0.95,
                    a: 1.0,
                },
            },
            Line {
                start: Point {
                    x: -128.88544,
                    y: -185.96533,
                },
                end: Point {
                    x: -174.0,
                    y: -183.0,
                },
                color: Color {
                    r: 0.0,
                    g: 0.89,
                    b: 0.19,
                    a: 1.0,
                },
            },
            Line {
                start: Point {
                    x: -128.88544,
                    y: -185.96533,
                },
                end: Point {
                    x: -122.88544,
                    y: -401.96533,
                },
                color: Color {
                    r: 0.9,
                    g: 0.16,
                    b: 0.22,
                    a: 1.0,
                },
            },
            Line {
                start: Point {
                    x: -122.88544,
                    y: -401.96533,
                },
                end: Point {
                    x: -23.885437,
                    y: -428.96533,
                },
                color: Color {
                    r: 0.0,
                    g: 0.47,
                    b: 0.95,
                    a: 1.0,
                },
            },
            Line {
                start: Point {
                    x: -23.885437,
                    y: -428.96533,
                },
                end: Point {
                    x: 470.9251,
                    y: -474.66516,
                },
                color: Color {
                    r: 0.0,
                    g: 0.89,
                    b: 0.19,
                    a: 1.0,
                },
            },
            Line {
                start: Point {
                    x: -209.51257,
                    y: 101.05615,
                },
                end: Point {
                    x: -160.0,
                    y: 167.0,
                },
                color: Color {
                    r: 0.9,
                    g: 0.16,
                    b: 0.22,
                    a: 1.0,
                },
            },
            Line {
                start: Point {
                    x: -209.51257,
                    y: 101.05615,
                },
                end: Point {
                    x: -169.51257,
                    y: 74.05615,
                },
                color: Color {
                    r: 0.0,
                    g: 0.47,
                    b: 0.95,
                    a: 1.0,
                },
            },
            Line {
                start: Point {
                    x: -169.51257,
                    y: 74.05615,
                },
                end: Point {
                    x: -116.51257,
                    y: 136.05615,
                },
                color: Color {
                    r: 0.0,
                    g: 0.89,
                    b: 0.19,
                    a: 1.0,
                },
            },
            Line {
                start: Point {
                    x: -116.51257,
                    y: 136.05615,
                },
                end: Point {
                    x: -77.51257,
                    y: 364.05615,
                },
                color: Color {
                    r: 0.9,
                    g: 0.16,
                    b: 0.22,
                    a: 1.0,
                },
            },
            Line {
                start: Point {
                    x: -77.51257,
                    y: 364.05615,
                },
                end: Point {
                    x: -18.512573,
                    y: 460.05615,
                },
                color: Color {
                    r: 0.0,
                    g: 0.47,
                    b: 0.95,
                    a: 1.0,
                },
            },
            Line {
                start: Point {
                    x: -18.512573,
                    y: 460.05615,
                },
                end: Point {
                    x: 57.65851,
                    y: 520.994,
                },
                color: Color {
                    r: 0.0,
                    g: 0.89,
                    b: 0.19,
                    a: 1.0,
                },
            },
            Line {
                start: Point {
                    x: 779.1825,
                    y: 259.93506,
                },
                end: Point {
                    x: 794.7189,
                    y: 295.596,
                },
                color: Color {
                    r: 0.9,
                    g: 0.16,
                    b: 0.22,
                    a: 1.0,
                },
            },
            Line {
                start: Point {
                    x: 815.1825,
                    y: 217.93506,
                },
                end: Point {
                    x: 779.1825,
                    y: 259.93506,
                },
                color: Color {
                    r: 0.0,
                    g: 0.47,
                    b: 0.95,
                    a: 1.0,
                },
            },
            Line {
                start: Point {
                    x: 815.1825,
                    y: 217.93506,
                },
                end: Point {
                    x: 906.5657,
                    y: -5.4526978,
                },
                color: Color {
                    r: 0.0,
                    g: 0.89,
                    b: 0.19,
                    a: 1.0,
                },
            },
        ],
        player: Player {
            position: Point {
                x: -139.3032,
                y: -40.031193,
            },
            angle: -25.149902,
        },
        selected: -1,
    }
}
