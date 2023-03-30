/*
 * Copyright 2023 William Swartzendruber
 *
 * To the extent possible under law, the person who associated CC0 with this file has waived all
 * copyright and related or neighboring rights to this file.
 *
 * You should have received a copy of the CC0 legalcode along with this work. If not, see
 * <http://creativecommons.org/publicdomain/zero/1.0/>.
 *
 * SPDX-License-Identifier: CC0-1.0
 */

use super::*;
use assert_approx_eq::assert_approx_eq;
use more_asserts::assert_le;

const DIFF: f64 = 0.0000000001;

#[test]
fn test_pq_e_to_dl() {

    //
    // These test vectors were generated by entering the PQ EOTF from ITU-R BT.2100-2 into
    // https://www.desmos.com/calculator and generating values.
    //

    assert_approx_eq!(pq_eotf(0.0), 0.0, DIFF);
    assert_approx_eq!(pq_eotf(0.1), 0.0000324565591464, DIFF);
    assert_approx_eq!(pq_eotf(0.2), 0.000242926329701, DIFF);
    assert_approx_eq!(pq_eotf(0.3), 0.00100382263105, DIFF);
    assert_approx_eq!(pq_eotf(0.4), 0.00324479178538, DIFF);
    assert_approx_eq!(pq_eotf(0.5), 0.00922457089941, DIFF);
    assert_approx_eq!(pq_eotf(0.6), 0.0244005192336, DIFF);
    assert_approx_eq!(pq_eotf(0.7), 0.0620879381648, DIFF);
    assert_approx_eq!(pq_eotf(0.8), 0.155517836429, DIFF);
    assert_approx_eq!(pq_eotf(0.9), 0.390564465283, DIFF);
    assert_approx_eq!(pq_eotf(1.0), 1.0, DIFF);
}

#[test]
fn test_pq_dl_to_e() {

    //
    // These test vectors were generated by entering the PQ iEOTF from ITU-R BT.2100-2 into
    // https://www.desmos.com/calculator and generating values.
    //

    assert_approx_eq!(pq_ieotf(0.0), 0.00000073095590258, DIFF);
    assert_approx_eq!(pq_ieotf(0.1), 0.751827096247, DIFF);
    assert_approx_eq!(pq_ieotf(0.2), 0.827424644859, DIFF);
    assert_approx_eq!(pq_ieotf(0.3), 0.87148644445, DIFF);
    assert_approx_eq!(pq_ieotf(0.4), 0.902572393311, DIFF);
    assert_approx_eq!(pq_ieotf(0.5), 0.926546704083, DIFF);
    assert_approx_eq!(pq_ieotf(0.6), 0.946028557801, DIFF);
    assert_approx_eq!(pq_ieotf(0.7), 0.962416136326, DIFF);
    assert_approx_eq!(pq_ieotf(0.8), 0.976543852322, DIFF);
    assert_approx_eq!(pq_ieotf(0.9), 0.988949526349, DIFF);
    assert_approx_eq!(pq_ieotf(1.0), 1.0, DIFF);
}

#[test]
fn test_hlg_sl_to_e() {

    //
    // These test vectors were generated by entering the HLG EOTF from ITU-R BT.2100-2 into
    // https://www.desmos.com/calculator and generating values.
    //

    assert_approx_eq!(hlg_oetf(0.0), 0.0, DIFF);
    assert_approx_eq!(hlg_oetf(0.1), 0.544089493962, DIFF);
    assert_approx_eq!(hlg_oetf(0.2), 0.693894268644, DIFF);
    assert_approx_eq!(hlg_oetf(0.3), 0.77425208582, DIFF);
    assert_approx_eq!(hlg_oetf(0.4), 0.82949728774, DIFF);
    assert_approx_eq!(hlg_oetf(0.5), 0.871643470874, DIFF);
    assert_approx_eq!(hlg_oetf(0.6), 0.905726960855, DIFF);
    assert_approx_eq!(hlg_oetf(0.7), 0.93434273564, DIFF);
    assert_approx_eq!(hlg_oetf(0.8), 0.95900494552, DIFF);
    assert_approx_eq!(hlg_oetf(0.9), 0.980674603875, DIFF);
    assert_approx_eq!(hlg_oetf(1.0), 0.999999995066, DIFF);
}

#[test]
fn test_hlg_dl_to_sl() {

    //
    // These test vectors were generated by entering the HLG iOOTF from ITU-R BT.2100-2 into
    // https://www.desmos.com/calculator and generating values.
    //

    let mut pixel;

    pixel = hlg_iootf(Pixel { red: 0.0, green: 0.0, blue: 0.0 });
    assert_approx_eq!(pixel.red, 0.0, DIFF);
    assert_approx_eq!(pixel.green, 0.0, DIFF);
    assert_approx_eq!(pixel.blue, 0.0, DIFF);

    pixel = hlg_iootf(Pixel { red: 0.33, green: 0.2, blue: 0.111 });
    assert_approx_eq!(pixel.red, 0.42193746045, DIFF);
    assert_approx_eq!(pixel.green, 0.255719673, DIFF);
    assert_approx_eq!(pixel.blue, 0.141924418515, DIFF);

    pixel = hlg_iootf(Pixel { red: 0.33, green: 0.2, blue: 0.222 });
    assert_approx_eq!(pixel.red, 0.419948243609, DIFF);
    assert_approx_eq!(pixel.green, 0.254514087036, DIFF);
    assert_approx_eq!(pixel.blue, 0.28251063661, DIFF);

    pixel = hlg_iootf(Pixel { red: 0.33, green: 0.4, blue: 0.333 });
    assert_approx_eq!(pixel.red, 0.388151567394, DIFF);
    assert_approx_eq!(pixel.green, 0.470486748357, DIFF);
    assert_approx_eq!(pixel.blue, 0.391680218007, DIFF);

    pixel = hlg_iootf(Pixel { red: 0.33, green: 0.4, blue: 0.444 });
    assert_approx_eq!(pixel.red, 0.387035297418, DIFF);
    assert_approx_eq!(pixel.green, 0.46913369384, DIFF);
    assert_approx_eq!(pixel.blue, 0.520738400162, DIFF);

    pixel = hlg_iootf(Pixel { red: 0.66, green: 0.6, blue: 0.555 });
    assert_approx_eq!(pixel.red, 0.716071106446, DIFF);
    assert_approx_eq!(pixel.green, 0.650973733133, DIFF);
    assert_approx_eq!(pixel.blue, 0.602150703148, DIFF);

    pixel = hlg_iootf(Pixel { red: 0.66, green: 0.6, blue: 0.666 });
    assert_approx_eq!(pixel.red, 0.7147977545, DIFF);
    assert_approx_eq!(pixel.green, 0.649816140455, DIFF);
    assert_approx_eq!(pixel.blue, 0.721295915905, DIFF);

    pixel = hlg_iootf(Pixel { red: 0.66, green: 0.8, blue: 0.777 });
    assert_approx_eq!(pixel.red, 0.690607972044, DIFF);
    assert_approx_eq!(pixel.green, 0.837100572175, DIFF);
    assert_approx_eq!(pixel.blue, 0.813033930725, DIFF);

    pixel = hlg_iootf(Pixel { red: 0.66, green: 0.8, blue: 0.888 });
    assert_approx_eq!(pixel.red, 0.689618500689, DIFF);
    assert_approx_eq!(pixel.green, 0.835901212956, DIFF);
    assert_approx_eq!(pixel.blue, 0.927850346382, DIFF);

    pixel = hlg_iootf(Pixel { red: 1.0, green: 1.0, blue: 1.0 });
    assert_approx_eq!(pixel.red, 1.0, DIFF);
    assert_approx_eq!(pixel.green, 1.0, DIFF);
    assert_approx_eq!(pixel.blue, 1.0, DIFF);
}

#[test]
fn test_hlg_compensation() {

    const SIZE: usize = 128;

    for b in 0..SIZE {
        for g in 0..SIZE {
            for r in 0..SIZE {

                let in_pixel = hlg_iootf(Pixel {
                    red: r as f64 / (SIZE - 1) as f64,
                    green: g as f64 / (SIZE - 1) as f64,
                    blue: b as f64 / (SIZE - 1) as f64,
                });
                let out_pixel = hlg_compensate(in_pixel);

                assert_approx_eq!(out_pixel.y(), in_pixel.y(), DIFF);
                assert_le!(out_pixel.red, 1.0);
                assert_le!(out_pixel.green, 1.0);
                assert_le!(out_pixel.blue, 1.0);
            }
        }
    }
}
