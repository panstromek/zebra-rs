use ::libc;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct OpeningDescriptor {
    pub name: *const libc::c_char,
    pub sequence: *const libc::c_char,
    pub hash_val1: libc::c_int,
    pub hash_val2: libc::c_int,
    pub level: libc::c_int,
}
/*
   opname.c

   Automatically created by OSF on Sun Nov 24 16:01:10 2002
*/
#[no_mangle]
pub static mut opening_list: [OpeningDescriptor; 76] =
    [{
         let mut init =
             OpeningDescriptor{name:
                                   b"Diagonal Opening\x00" as *const u8 as
                                       *const libc::c_char,
                               sequence:
                                   b"C4c3\x00" as *const u8 as
                                       *const libc::c_char,
                               hash_val1: 1659994584 as libc::c_int,
                               hash_val2: 1512627031 as libc::c_int,
                               level: 0 as libc::c_int,};
         init
     },
     {
         let mut init =
             OpeningDescriptor{name:
                                   b"X-square Opening\x00" as *const u8 as
                                       *const libc::c_char,
                               sequence:
                                   b"C4c3D3c5B2\x00" as *const u8 as
                                       *const libc::c_char,
                               hash_val1: 2077036941 as libc::c_int,
                               hash_val2: 1593974113 as libc::c_int,
                               level: 1 as libc::c_int,};
         init
     },
     {
         let mut init =
             OpeningDescriptor{name:
                                   b"Snake / Peasant\x00" as *const u8 as
                                       *const libc::c_char,
                               sequence:
                                   b"C4c3D3c5B3\x00" as *const u8 as
                                       *const libc::c_char,
                               hash_val1: 1827743487 as libc::c_int,
                               hash_val2: 1105897370 as libc::c_int,
                               level: 1 as libc::c_int,};
         init
     },
     {
         let mut init =
             OpeningDescriptor{name:
                                   b"Pyramid / Checkerboarding Peasant\x00" as
                                       *const u8 as *const libc::c_char,
                               sequence:
                                   b"C4c3D3c5B3f4B5b4C6d6F5\x00" as *const u8
                                       as *const libc::c_char,
                               hash_val1: 1722362445 as libc::c_int,
                               hash_val2: 1515117211 as libc::c_int,
                               level: 2 as libc::c_int,};
         init
     },
     {
         let mut init =
             OpeningDescriptor{name:
                                   b"Heath / Tobidashi\x00" as *const u8 as
                                       *const libc::c_char,
                               sequence:
                                   b"C4c3D3c5B4\x00" as *const u8 as
                                       *const libc::c_char,
                               hash_val1: 1803937853 as libc::c_int,
                               hash_val2: 850730059 as libc::c_int,
                               level: 1 as libc::c_int,};
         init
     },
     {
         let mut init =
             OpeningDescriptor{name:
                                   b"Mimura variation II\x00" as *const u8 as
                                       *const libc::c_char,
                               sequence:
                                   b"C4c3D3c5B4d2C2f4D6c6F5e6F7\x00" as
                                       *const u8 as *const libc::c_char,
                               hash_val1: 1292601516 as libc::c_int,
                               hash_val2: 333200793 as libc::c_int,
                               level: 2 as libc::c_int,};
         init
     },
     {
         let mut init =
             OpeningDescriptor{name:
                                   b"Heath-Bat\x00" as *const u8 as
                                       *const libc::c_char,
                               sequence:
                                   b"C4c3D3c5B4d2D6\x00" as *const u8 as
                                       *const libc::c_char,
                               hash_val1: 2045341493 as libc::c_int,
                               hash_val2: 1565190299 as libc::c_int,
                               level: 2 as libc::c_int,};
         init
     },
     {
         let mut init =
             OpeningDescriptor{name:
                                   b"Iwasaki variation\x00" as *const u8 as
                                       *const libc::c_char,
                               sequence:
                                   b"C4c3D3c5B4d2E2\x00" as *const u8 as
                                       *const libc::c_char,
                               hash_val1: 1303715030 as libc::c_int,
                               hash_val2: 988675731 as libc::c_int,
                               level: 2 as libc::c_int,};
         init
     },
     {
         let mut init =
             OpeningDescriptor{name:
                                   b"Heath-Chimney\x00" as *const u8 as
                                       *const libc::c_char,
                               sequence:
                                   b"C4c3D3c5B4e3\x00" as *const u8 as
                                       *const libc::c_char,
                               hash_val1: 991304461 as libc::c_int,
                               hash_val2: 1396667520 as libc::c_int,
                               level: 2 as libc::c_int,};
         init
     },
     {
         let mut init =
             OpeningDescriptor{name:
                                   b"Raccoon Dog\x00" as *const u8 as
                                       *const libc::c_char,
                               sequence:
                                   b"C4c3D3c5B5\x00" as *const u8 as
                                       *const libc::c_char,
                               hash_val1: 1867091946 as libc::c_int,
                               hash_val2: 1939139376 as libc::c_int,
                               level: 1 as libc::c_int,};
         init
     },
     {
         let mut init =
             OpeningDescriptor{name:
                                   b"Rocket\x00" as *const u8 as
                                       *const libc::c_char,
                               sequence:
                                   b"C4c3D3c5B6\x00" as *const u8 as
                                       *const libc::c_char,
                               hash_val1: 2104829089 as libc::c_int,
                               hash_val2: 1022272519 as libc::c_int,
                               level: 1 as libc::c_int,};
         init
     },
     {
         let mut init =
             OpeningDescriptor{name:
                                   b"Hamilton\x00" as *const u8 as
                                       *const libc::c_char,
                               sequence:
                                   b"C4c3D3c5B6c6B5\x00" as *const u8 as
                                       *const libc::c_char,
                               hash_val1: 258232499 as libc::c_int,
                               hash_val2: 2026592390 as libc::c_int,
                               level: 2 as libc::c_int,};
         init
     },
     {
         let mut init =
             OpeningDescriptor{name:
                                   b"Lollipop\x00" as *const u8 as
                                       *const libc::c_char,
                               sequence:
                                   b"C4c3D3c5B6e3\x00" as *const u8 as
                                       *const libc::c_char,
                               hash_val1: 1858297015 as libc::c_int,
                               hash_val2: 885645857 as libc::c_int,
                               level: 2 as libc::c_int,};
         init
     },
     {
         let mut init =
             OpeningDescriptor{name:
                                   b"Cow\x00" as *const u8 as
                                       *const libc::c_char,
                               sequence:
                                   b"C4c3D3c5D6\x00" as *const u8 as
                                       *const libc::c_char,
                               hash_val1: 1481949922 as libc::c_int,
                               hash_val2: 1585684585 as libc::c_int,
                               level: 1 as libc::c_int,};
         init
     },
     {
         let mut init =
             OpeningDescriptor{name:
                                   b"Chimney\x00" as *const u8 as
                                       *const libc::c_char,
                               sequence:
                                   b"C4c3D3c5D6e3\x00" as *const u8 as
                                       *const libc::c_char,
                               hash_val1: 1693540789 as libc::c_int,
                               hash_val2: 1417640656 as libc::c_int,
                               level: 2 as libc::c_int,};
         init
     },
     {
         let mut init =
             OpeningDescriptor{name:
                                   b"Cow Bat / Bat / Cambridge\x00" as
                                       *const u8 as *const libc::c_char,
                               sequence:
                                   b"C4c3D3c5D6f4B4\x00" as *const u8 as
                                       *const libc::c_char,
                               hash_val1: 1263026890 as libc::c_int,
                               hash_val2: 1568764298 as libc::c_int,
                               level: 2 as libc::c_int,};
         init
     },
     {
         let mut init =
             OpeningDescriptor{name:
                                   b"Bat (Piau Continuation 2)\x00" as
                                       *const u8 as *const libc::c_char,
                               sequence:
                                   b"C4c3D3c5D6f4B4b6B5c6B3\x00" as *const u8
                                       as *const libc::c_char,
                               hash_val1: 1742741974 as libc::c_int,
                               hash_val2: 1194404142 as libc::c_int,
                               level: 3 as libc::c_int,};
         init
     },
     {
         let mut init =
             OpeningDescriptor{name:
                                   b"Melnikov / Bat (Piau Continuation 1)\x00"
                                       as *const u8 as *const libc::c_char,
                               sequence:
                                   b"C4c3D3c5D6f4B4b6B5c6F5\x00" as *const u8
                                       as *const libc::c_char,
                               hash_val1: 1054417007 as libc::c_int,
                               hash_val2: 389015849 as libc::c_int,
                               level: 3 as libc::c_int,};
         init
     },
     {
         let mut init =
             OpeningDescriptor{name:
                                   b"Bat (Kling Continuation)\x00" as
                                       *const u8 as *const libc::c_char,
                               sequence:
                                   b"C4c3D3c5D6f4B4c6B5b3B6e3C2a4A5a6D2\x00"
                                       as *const u8 as *const libc::c_char,
                               hash_val1: 2070507873 as libc::c_int,
                               hash_val2: 1967369675 as libc::c_int,
                               level: 3 as libc::c_int,};
         init
     },
     {
         let mut init =
             OpeningDescriptor{name:
                                   b"Bat (Kling Alternative)\x00" as *const u8
                                       as *const libc::c_char,
                               sequence:
                                   b"C4c3D3c5D6f4B4e3B3\x00" as *const u8 as
                                       *const libc::c_char,
                               hash_val1: 2147155449 as libc::c_int,
                               hash_val2: 1110421450 as libc::c_int,
                               level: 3 as libc::c_int,};
         init
     },
     {
         let mut init =
             OpeningDescriptor{name:
                                   b"Rose-v-Toth\x00" as *const u8 as
                                       *const libc::c_char,
                               sequence:
                                   b"C4c3D3c5D6f4F5\x00" as *const u8 as
                                       *const libc::c_char,
                               hash_val1: 1775822193 as libc::c_int,
                               hash_val2: 849039804 as libc::c_int,
                               level: 2 as libc::c_int,};
         init
     },
     {
         let mut init =
             OpeningDescriptor{name:
                                   b"Tanida\x00" as *const u8 as
                                       *const libc::c_char,
                               sequence:
                                   b"C4c3D3c5D6f4F5d2\x00" as *const u8 as
                                       *const libc::c_char,
                               hash_val1: 1437486538 as libc::c_int,
                               hash_val2: 1419370563 as libc::c_int,
                               level: 3 as libc::c_int,};
         init
     },
     {
         let mut init =
             OpeningDescriptor{name:
                                   b"Aircraft / Feldborg\x00" as *const u8 as
                                       *const libc::c_char,
                               sequence:
                                   b"C4c3D3c5D6f4F5d2B5\x00" as *const u8 as
                                       *const libc::c_char,
                               hash_val1: 1716567246 as libc::c_int,
                               hash_val2: 572996646 as libc::c_int,
                               level: 4 as libc::c_int,};
         init
     },
     {
         let mut init =
             OpeningDescriptor{name:
                                   b"Sailboat\x00" as *const u8 as
                                       *const libc::c_char,
                               sequence:
                                   b"C4c3D3c5D6f4F5d2G4d7\x00" as *const u8 as
                                       *const libc::c_char,
                               hash_val1: 1924813628 as libc::c_int,
                               hash_val2: 167558475 as libc::c_int,
                               level: 4 as libc::c_int,};
         init
     },
     {
         let mut init =
             OpeningDescriptor{name:
                                   b"Maruoka\x00" as *const u8 as
                                       *const libc::c_char,
                               sequence:
                                   b"C4c3D3c5D6f4F5e6C6d7\x00" as *const u8 as
                                       *const libc::c_char,
                               hash_val1: 1590274807 as libc::c_int,
                               hash_val2: 1176436578 as libc::c_int,
                               level: 3 as libc::c_int,};
         init
     },
     {
         let mut init =
             OpeningDescriptor{name:
                                   b"Landau\x00" as *const u8 as
                                       *const libc::c_char,
                               sequence:
                                   b"C4c3D3c5D6f4F5e6F6\x00" as *const u8 as
                                       *const libc::c_char,
                               hash_val1: 1835140035 as libc::c_int,
                               hash_val2: 1943982505 as libc::c_int,
                               level: 3 as libc::c_int,};
         init
     },
     {
         let mut init =
             OpeningDescriptor{name:
                                   b"Buffalo / Kenichi Variation\x00" as
                                       *const u8 as *const libc::c_char,
                               sequence:
                                   b"C4c3D3c5F6\x00" as *const u8 as
                                       *const libc::c_char,
                               hash_val1: 1016471071 as libc::c_int,
                               hash_val2: 1350965242 as libc::c_int,
                               level: 1 as libc::c_int,};
         init
     },
     {
         let mut init =
             OpeningDescriptor{name:
                                   b"Maruoka Buffalo\x00" as *const u8 as
                                       *const libc::c_char,
                               sequence:
                                   b"C4c3D3c5F6e2C6\x00" as *const u8 as
                                       *const libc::c_char,
                               hash_val1: 2069772984 as libc::c_int,
                               hash_val2: 1663578667 as libc::c_int,
                               level: 2 as libc::c_int,};
         init
     },
     {
         let mut init =
             OpeningDescriptor{name:
                                   b"Tanida Buffalo\x00" as *const u8 as
                                       *const libc::c_char,
                               sequence:
                                   b"C4c3D3c5F6e3C6f5F4g5\x00" as *const u8 as
                                       *const libc::c_char,
                               hash_val1: 1663398250 as libc::c_int,
                               hash_val2: 426197306 as libc::c_int,
                               level: 2 as libc::c_int,};
         init
     },
     {
         let mut init =
             OpeningDescriptor{name:
                                   b"Hokuriku Buffalo\x00" as *const u8 as
                                       *const libc::c_char,
                               sequence:
                                   b"C4c3D3c5F6f5\x00" as *const u8 as
                                       *const libc::c_char,
                               hash_val1: 2009716880 as libc::c_int,
                               hash_val2: 2108488280 as libc::c_int,
                               level: 2 as libc::c_int,};
         init
     },
     {
         let mut init =
             OpeningDescriptor{name:
                                   b"Lysons\x00" as *const u8 as
                                       *const libc::c_char,
                               sequence:
                                   b"C4c3D3e3C2c6\x00" as *const u8 as
                                       *const libc::c_char,
                               hash_val1: 1997241503 as libc::c_int,
                               hash_val2: 154907557 as libc::c_int,
                               level: 1 as libc::c_int,};
         init
     },
     {
         let mut init =
             OpeningDescriptor{name:
                                   b"Wing Variation\x00" as *const u8 as
                                       *const libc::c_char,
                               sequence:
                                   b"C4c3E6c5\x00" as *const u8 as
                                       *const libc::c_char,
                               hash_val1: 1351687293 as libc::c_int,
                               hash_val2: 1211048733 as libc::c_int,
                               level: 1 as libc::c_int,};
         init
     },
     {
         let mut init =
             OpeningDescriptor{name:
                                   b"Semi-Wing Variation\x00" as *const u8 as
                                       *const libc::c_char,
                               sequence:
                                   b"C4c3F5c5\x00" as *const u8 as
                                       *const libc::c_char,
                               hash_val1: 1824270948 as libc::c_int,
                               hash_val2: 1134716060 as libc::c_int,
                               level: 1 as libc::c_int,};
         init
     },
     {
         let mut init =
             OpeningDescriptor{name:
                                   b"Parallel Opening\x00" as *const u8 as
                                       *const libc::c_char,
                               sequence:
                                   b"C4c5\x00" as *const u8 as
                                       *const libc::c_char,
                               hash_val1: 829656535 as libc::c_int,
                               hash_val2: 1764123977 as libc::c_int,
                               level: 0 as libc::c_int,};
         init
     },
     {
         let mut init =
             OpeningDescriptor{name:
                                   b"Perpendicular Opening\x00" as *const u8
                                       as *const libc::c_char,
                               sequence:
                                   b"C4e3\x00" as *const u8 as
                                       *const libc::c_char,
                               hash_val1: 1851744936 as libc::c_int,
                               hash_val2: 1258123872 as libc::c_int,
                               level: 0 as libc::c_int,};
         init
     },
     {
         let mut init =
             OpeningDescriptor{name:
                                   b"Shaman / Danish\x00" as *const u8 as
                                       *const libc::c_char,
                               sequence:
                                   b"C4e3F4c5D6f3C6\x00" as *const u8 as
                                       *const libc::c_char,
                               hash_val1: 1590510995 as libc::c_int,
                               hash_val2: 657803652 as libc::c_int,
                               level: 1 as libc::c_int,};
         init
     },
     {
         let mut init =
             OpeningDescriptor{name:
                                   b"Inoue\x00" as *const u8 as
                                       *const libc::c_char,
                               sequence:
                                   b"C4e3F4c5D6f3D3\x00" as *const u8 as
                                       *const libc::c_char,
                               hash_val1: 1843823936 as libc::c_int,
                               hash_val2: 392387661 as libc::c_int,
                               level: 1 as libc::c_int,};
         init
     },
     {
         let mut init =
             OpeningDescriptor{name:
                                   b"Iago\x00" as *const u8 as
                                       *const libc::c_char,
                               sequence:
                                   b"C4e3F4c5D6f3D3c3\x00" as *const u8 as
                                       *const libc::c_char,
                               hash_val1: 1799555712 as libc::c_int,
                               hash_val2: 1106622442 as libc::c_int,
                               level: 2 as libc::c_int,};
         init
     },
     {
         let mut init =
             OpeningDescriptor{name:
                                   b"Bhagat\x00" as *const u8 as
                                       *const libc::c_char,
                               sequence:
                                   b"C4e3F4c5D6f3E2\x00" as *const u8 as
                                       *const libc::c_char,
                               hash_val1: 1281256861 as libc::c_int,
                               hash_val2: 1711005289 as libc::c_int,
                               level: 1 as libc::c_int,};
         init
     },
     {
         let mut init =
             OpeningDescriptor{name:
                                   b"Rose\x00" as *const u8 as
                                       *const libc::c_char,
                               sequence:
                                   b"C4e3F4c5D6f3E6c3D3e2\x00" as *const u8 as
                                       *const libc::c_char,
                               hash_val1: 2026816578 as libc::c_int,
                               hash_val2: 1865116048 as libc::c_int,
                               level: 1 as libc::c_int,};
         init
     },
     {
         let mut init =
             OpeningDescriptor{name:
                                   b"Flat\x00" as *const u8 as
                                       *const libc::c_char,
                               sequence:
                                   b"C4e3F4c5D6f3E6c3D3e2B5\x00" as *const u8
                                       as *const libc::c_char,
                               hash_val1: 2035993944 as libc::c_int,
                               hash_val2: 620991891 as libc::c_int,
                               level: 2 as libc::c_int,};
         init
     },
     {
         let mut init =
             OpeningDescriptor{name:
                                   b"Rotating Flat\x00" as *const u8 as
                                       *const libc::c_char,
                               sequence:
                                   b"C4e3F4c5D6f3E6c3D3e2B5f5\x00" as
                                       *const u8 as *const libc::c_char,
                               hash_val1: 1039898066 as libc::c_int,
                               hash_val2: 1320292744 as libc::c_int,
                               level: 3 as libc::c_int,};
         init
     },
     {
         let mut init =
             OpeningDescriptor{name:
                                   b"Murakami Variation\x00" as *const u8 as
                                       *const libc::c_char,
                               sequence:
                                   b"C4e3F4c5D6f3E6c3D3e2B5f5B3\x00" as
                                       *const u8 as *const libc::c_char,
                               hash_val1: 1858376787 as libc::c_int,
                               hash_val2: 514175582 as libc::c_int,
                               level: 4 as libc::c_int,};
         init
     },
     {
         let mut init =
             OpeningDescriptor{name:
                                   b"Rotating Flat (Kling Continuation)\x00"
                                       as *const u8 as *const libc::c_char,
                               sequence:
                                   b"C4e3F4c5D6f3E6c3D3e2B5f5B4f6C2e7D2c7\x00"
                                       as *const u8 as *const libc::c_char,
                               hash_val1: 2117167674 as libc::c_int,
                               hash_val2: 1013825450 as libc::c_int,
                               level: 4 as libc::c_int,};
         init
     },
     {
         let mut init =
             OpeningDescriptor{name:
                                   b"Rose-Birth\x00" as *const u8 as
                                       *const libc::c_char,
                               sequence:
                                   b"C4e3F4c5D6f3E6c3D3e2B6f5\x00" as
                                       *const u8 as *const libc::c_char,
                               hash_val1: 587782303 as libc::c_int,
                               hash_val2: 696276737 as libc::c_int,
                               level: 2 as libc::c_int,};
         init
     },
     {
         let mut init =
             OpeningDescriptor{name:
                                   b"Brightstein\x00" as *const u8 as
                                       *const libc::c_char,
                               sequence:
                                   b"C4e3F4c5D6f3E6c3D3e2B6f5B4f6G5d7\x00" as
                                       *const u8 as *const libc::c_char,
                               hash_val1: 1706436162 as libc::c_int,
                               hash_val2: 921464410 as libc::c_int,
                               level: 3 as libc::c_int,};
         init
     },
     {
         let mut init =
             OpeningDescriptor{name:
                                   b"Rose-birdie / Rose-Tamenori\x00" as
                                       *const u8 as *const libc::c_char,
                               sequence:
                                   b"C4e3F4c5D6f3E6c3D3e2B6f5G5\x00" as
                                       *const u8 as *const libc::c_char,
                               hash_val1: 1738295284 as libc::c_int,
                               hash_val2: 1228019111 as libc::c_int,
                               level: 3 as libc::c_int,};
         init
     },
     {
         let mut init =
             OpeningDescriptor{name:
                                   b"Rose-Tamenori-Kling\x00" as *const u8 as
                                       *const libc::c_char,
                               sequence:
                                   b"C4e3F4c5D6f3E6c3D3e2B6f5G5f6\x00" as
                                       *const u8 as *const libc::c_char,
                               hash_val1: 2068590107 as libc::c_int,
                               hash_val2: 1121147553 as libc::c_int,
                               level: 4 as libc::c_int,};
         init
     },
     {
         let mut init =
             OpeningDescriptor{name:
                                   b"Greenberg / Dawg\x00" as *const u8 as
                                       *const libc::c_char,
                               sequence:
                                   b"C4e3F4c5D6f3E6c3D3e2D2\x00" as *const u8
                                       as *const libc::c_char,
                               hash_val1: 1957523527 as libc::c_int,
                               hash_val2: 2126554611 as libc::c_int,
                               level: 2 as libc::c_int,};
         init
     },
     {
         let mut init =
             OpeningDescriptor{name:
                                   b"Ralle\x00" as *const u8 as
                                       *const libc::c_char,
                               sequence:
                                   b"C4e3F4c5D6f3E6c6\x00" as *const u8 as
                                       *const libc::c_char,
                               hash_val1: 1034822540 as libc::c_int,
                               hash_val2: 153671416 as libc::c_int,
                               level: 1 as libc::c_int,};
         init
     },
     {
         let mut init =
             OpeningDescriptor{name:
                                   b"Horse\x00" as *const u8 as
                                       *const libc::c_char,
                               sequence:
                                   b"C4e3F4c5E6\x00" as *const u8 as
                                       *const libc::c_char,
                               hash_val1: 1282505844 as libc::c_int,
                               hash_val2: 486114351 as libc::c_int,
                               level: 1 as libc::c_int,};
         init
     },
     {
         let mut init =
             OpeningDescriptor{name:
                                   b"Ganglion / No-Cat\x00" as *const u8 as
                                       *const libc::c_char,
                               sequence:
                                   b"C4e3F5b4\x00" as *const u8 as
                                       *const libc::c_char,
                               hash_val1: 2146248322 as libc::c_int,
                               hash_val2: 614307064 as libc::c_int,
                               level: 1 as libc::c_int,};
         init
     },
     {
         let mut init =
             OpeningDescriptor{name:
                                   b"Swallow\x00" as *const u8 as
                                       *const libc::c_char,
                               sequence:
                                   b"C4e3F5b4F3\x00" as *const u8 as
                                       *const libc::c_char,
                               hash_val1: 1348643389 as libc::c_int,
                               hash_val2: 243952072 as libc::c_int,
                               level: 2 as libc::c_int,};
         init
     },
     {
         let mut init =
             OpeningDescriptor{name:
                                   b"No-Cat (Continuation)\x00" as *const u8
                                       as *const libc::c_char,
                               sequence:
                                   b"C4e3F5b4F3f4E2e6G5f6D6c6\x00" as
                                       *const u8 as *const libc::c_char,
                               hash_val1: 1659204929 as libc::c_int,
                               hash_val2: 913839576 as libc::c_int,
                               level: 3 as libc::c_int,};
         init
     },
     {
         let mut init =
             OpeningDescriptor{name:
                                   b"Italian\x00" as *const u8 as
                                       *const libc::c_char,
                               sequence:
                                   b"C4e3F5e6D3\x00" as *const u8 as
                                       *const libc::c_char,
                               hash_val1: 1829312902 as libc::c_int,
                               hash_val2: 1703714145 as libc::c_int,
                               level: 1 as libc::c_int,};
         init
     },
     {
         let mut init =
             OpeningDescriptor{name:
                                   b"Cat\x00" as *const u8 as
                                       *const libc::c_char,
                               sequence:
                                   b"C4e3F5e6F4\x00" as *const u8 as
                                       *const libc::c_char,
                               hash_val1: 1316058318 as libc::c_int,
                               hash_val2: 2050471550 as libc::c_int,
                               level: 1 as libc::c_int,};
         init
     },
     {
         let mut init =
             OpeningDescriptor{name:
                                   b"Sakaguchi\x00" as *const u8 as
                                       *const libc::c_char,
                               sequence:
                                   b"C4e3F5e6F4c5D6c6F7f3\x00" as *const u8 as
                                       *const libc::c_char,
                               hash_val1: 1100639514 as libc::c_int,
                               hash_val2: 1030214846 as libc::c_int,
                               level: 2 as libc::c_int,};
         init
     },
     {
         let mut init =
             OpeningDescriptor{name:
                                   b"Berner\x00" as *const u8 as
                                       *const libc::c_char,
                               sequence:
                                   b"C4e3F5e6F4c5D6c6F7g5G6\x00" as *const u8
                                       as *const libc::c_char,
                               hash_val1: 1607375676 as libc::c_int,
                               hash_val2: 1170521442 as libc::c_int,
                               level: 2 as libc::c_int,};
         init
     },
     {
         let mut init =
             OpeningDescriptor{name:
                                   b"Bent ganglion\x00" as *const u8 as
                                       *const libc::c_char,
                               sequence:
                                   b"C4e3F6b4\x00" as *const u8 as
                                       *const libc::c_char,
                               hash_val1: 2077965605 as libc::c_int,
                               hash_val2: 752551187 as libc::c_int,
                               level: 1 as libc::c_int,};
         init
     },
     {
         let mut init =
             OpeningDescriptor{name:
                                   b"Tiger\x00" as *const u8 as
                                       *const libc::c_char,
                               sequence:
                                   b"C4e3F6e6F5\x00" as *const u8 as
                                       *const libc::c_char,
                               hash_val1: 1497408290 as libc::c_int,
                               hash_val2: 1946403171 as libc::c_int,
                               level: 1 as libc::c_int,};
         init
     },
     {
         let mut init =
             OpeningDescriptor{name:
                                   b"Stephenson\x00" as *const u8 as
                                       *const libc::c_char,
                               sequence:
                                   b"C4e3F6e6F5c5C3\x00" as *const u8 as
                                       *const libc::c_char,
                               hash_val1: 2001484961 as libc::c_int,
                               hash_val2: 997291518 as libc::c_int,
                               level: 2 as libc::c_int,};
         init
     },
     {
         let mut init =
             OpeningDescriptor{name:
                                   b"No-Kung\x00" as *const u8 as
                                       *const libc::c_char,
                               sequence:
                                   b"C4e3F6e6F5c5C3b4\x00" as *const u8 as
                                       *const libc::c_char,
                               hash_val1: 1408052601 as libc::c_int,
                               hash_val2: 202339595 as libc::c_int,
                               level: 3 as libc::c_int,};
         init
     },
     {
         let mut init =
             OpeningDescriptor{name:
                                   b"No-Kung (Continuation)\x00" as *const u8
                                       as *const libc::c_char,
                               sequence:
                                   b"C4e3F6e6F5c5C3b4D6c6B5a6B6c7\x00" as
                                       *const u8 as *const libc::c_char,
                               hash_val1: 1936197448 as libc::c_int,
                               hash_val2: 765630338 as libc::c_int,
                               level: 4 as libc::c_int,};
         init
     },
     {
         let mut init =
             OpeningDescriptor{name:
                                   b"Comp\'Oth\x00" as *const u8 as
                                       *const libc::c_char,
                               sequence:
                                   b"C4e3F6e6F5c5C3c6\x00" as *const u8 as
                                       *const libc::c_char,
                               hash_val1: 1088603028 as libc::c_int,
                               hash_val2: 40702271 as libc::c_int,
                               level: 3 as libc::c_int,};
         init
     },
     {
         let mut init =
             OpeningDescriptor{name:
                                   b"F.A.T. draw\x00" as *const u8 as
                                       *const libc::c_char,
                               sequence:
                                   b"C4e3F6e6F5c5C3c6D3d2E2b3C1c2B4a3A5b5A6a4A2\x00"
                                       as *const u8 as *const libc::c_char,
                               hash_val1: 2133533603 as libc::c_int,
                               hash_val2: 578375933 as libc::c_int,
                               level: 4 as libc::c_int,};
         init
     },
     {
         let mut init =
             OpeningDescriptor{name:
                                   b"Lightning bolt\x00" as *const u8 as
                                       *const libc::c_char,
                               sequence:
                                   b"C4e3F6e6F5c5C3c6D6\x00" as *const u8 as
                                       *const libc::c_char,
                               hash_val1: 1681415718 as libc::c_int,
                               hash_val2: 1804439570 as libc::c_int,
                               level: 4 as libc::c_int,};
         init
     },
     {
         let mut init =
             OpeningDescriptor{name:
                                   b"Kung\x00" as *const u8 as
                                       *const libc::c_char,
                               sequence:
                                   b"C4e3F6e6F5c5C3g5\x00" as *const u8 as
                                       *const libc::c_char,
                               hash_val1: 1161814519 as libc::c_int,
                               hash_val2: 227413804 as libc::c_int,
                               level: 3 as libc::c_int,};
         init
     },
     {
         let mut init =
             OpeningDescriptor{name:
                                   b"Leader\'s Tiger\x00" as *const u8 as
                                       *const libc::c_char,
                               sequence:
                                   b"C4e3F6e6F5c5D3\x00" as *const u8 as
                                       *const libc::c_char,
                               hash_val1: 2080541149 as libc::c_int,
                               hash_val2: 1461910541 as libc::c_int,
                               level: 2 as libc::c_int,};
         init
     },
     {
         let mut init =
             OpeningDescriptor{name:
                                   b"Brightwell\x00" as *const u8 as
                                       *const libc::c_char,
                               sequence:
                                   b"C4e3F6e6F5c5D6\x00" as *const u8 as
                                       *const libc::c_char,
                               hash_val1: 1496773054 as libc::c_int,
                               hash_val2: 659326594 as libc::c_int,
                               level: 2 as libc::c_int,};
         init
     },
     {
         let mut init =
             OpeningDescriptor{name:
                                   b"Ishii\x00" as *const u8 as
                                       *const libc::c_char,
                               sequence:
                                   b"C4e3F6e6F5c5F4g5G4f3C6d3D6\x00" as
                                       *const u8 as *const libc::c_char,
                               hash_val1: 2004859868 as libc::c_int,
                               hash_val2: 1457480717 as libc::c_int,
                               level: 2 as libc::c_int,};
         init
     },
     {
         let mut init =
             OpeningDescriptor{name:
                                   b"Mainline Tiger\x00" as *const u8 as
                                       *const libc::c_char,
                               sequence:
                                   b"C4e3F6e6F5c5F4g5G4f3C6d3D6b3C3b4E2b6\x00"
                                       as *const u8 as *const libc::c_char,
                               hash_val1: 1053928921 as libc::c_int,
                               hash_val2: 59668871 as libc::c_int,
                               level: 3 as libc::c_int,};
         init
     },
     {
         let mut init =
             OpeningDescriptor{name:
                                   b"Rose-Bill\x00" as *const u8 as
                                       *const libc::c_char,
                               sequence:
                                   b"C4e3F6e6F5c5F4g6F7\x00" as *const u8 as
                                       *const libc::c_char,
                               hash_val1: 1514352425 as libc::c_int,
                               hash_val2: 208645683 as libc::c_int,
                               level: 2 as libc::c_int,};
         init
     },
     {
         let mut init =
             OpeningDescriptor{name:
                                   b"Central Rose-Bill / Dead draw\x00" as
                                       *const u8 as *const libc::c_char,
                               sequence:
                                   b"C4e3F6e6F5c5F4g6F7g5\x00" as *const u8 as
                                       *const libc::c_char,
                               hash_val1: 1439795892 as libc::c_int,
                               hash_val2: 1828320679 as libc::c_int,
                               level: 3 as libc::c_int,};
         init
     },
     {
         let mut init =
             OpeningDescriptor{name:
                                   b"Tamenori\x00" as *const u8 as
                                       *const libc::c_char,
                               sequence:
                                   b"C4e3F6e6F5c5F4g6F7d3\x00" as *const u8 as
                                       *const libc::c_char,
                               hash_val1: 2100764374 as libc::c_int,
                               hash_val2: 1995376218 as libc::c_int,
                               level: 3 as libc::c_int,};
         init
     },
     {
         let mut init =
             OpeningDescriptor{name:
                                   b"Aubrey / Tanaka\x00" as *const u8 as
                                       *const libc::c_char,
                               sequence:
                                   b"C4e3F6e6F5g6\x00" as *const u8 as
                                       *const libc::c_char,
                               hash_val1: 1786050413 as libc::c_int,
                               hash_val2: 992498256 as libc::c_int,
                               level: 2 as libc::c_int,};
         init
     },
     {
         let mut init =
             OpeningDescriptor{name:
                                   b"Aubrey (Feldborg continuation)\x00" as
                                       *const u8 as *const libc::c_char,
                               sequence:
                                   b"C4e3F6e6F5g6E7c5\x00" as *const u8 as
                                       *const libc::c_char,
                               hash_val1: 1471486561 as libc::c_int,
                               hash_val2: 551414086 as libc::c_int,
                               level: 3 as libc::c_int,};
         init
     }];
