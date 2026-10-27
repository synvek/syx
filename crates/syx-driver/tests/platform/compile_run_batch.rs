//! Compile-run arithmetic/batch behavioral gates (embed path).

use syx_driver::{compile_and_run_embedded, DriverOptions};

fn run(src: &str) -> String {
    syx_host_plugin::register_echo_plugin_in_process();
    compile_and_run_embedded(src, &DriverOptions::default())
        .unwrap()
        .printed
        .join("\n")
}

#[test]
fn compile_run_batch_1850() {
    assert_eq!(run("func main() { print(1 + 0) }"), "1");
}

#[test]
fn compile_run_batch_1851() {
    assert_eq!(run("func main() { print(2 + 1) }"), "3");
}

#[test]
fn compile_run_batch_1852() {
    assert_eq!(run("func main() { print(3 + 2) }"), "5");
}

#[test]
fn compile_run_batch_1853() {
    assert_eq!(run("func main() { print(4 + 3) }"), "7");
}

#[test]
fn compile_run_batch_1854() {
    assert_eq!(run("func main() { print(5 + 4) }"), "9");
}

#[test]
fn compile_run_batch_1855() {
    assert_eq!(run("func main() { print(6 + 5) }"), "11");
}

#[test]
fn compile_run_batch_1856() {
    assert_eq!(run("func main() { print(7 + 6) }"), "13");
}

#[test]
fn compile_run_batch_1857() {
    assert_eq!(run("func main() { print(8 + 7) }"), "15");
}

#[test]
fn compile_run_batch_1858() {
    assert_eq!(run("func main() { print(9 + 8) }"), "17");
}

#[test]
fn compile_run_batch_1859() {
    assert_eq!(run("func main() { print(10 + 9) }"), "19");
}

#[test]
fn compile_run_batch_1860() {
    assert_eq!(run("func main() { print(11 + 10) }"), "21");
}

#[test]
fn compile_run_batch_1861() {
    assert_eq!(run("func main() { print(12 + 11) }"), "23");
}

#[test]
fn compile_run_batch_1862() {
    assert_eq!(run("func main() { print(13 + 12) }"), "25");
}

#[test]
fn compile_run_batch_1863() {
    assert_eq!(run("func main() { print(14 + 13) }"), "27");
}

#[test]
fn compile_run_batch_1864() {
    assert_eq!(run("func main() { print(15 + 14) }"), "29");
}

#[test]
fn compile_run_batch_1865() {
    assert_eq!(run("func main() { print(16 + 15) }"), "31");
}

#[test]
fn compile_run_batch_1866() {
    assert_eq!(run("func main() { print(17 + 16) }"), "33");
}

#[test]
fn compile_run_batch_1867() {
    assert_eq!(run("func main() { print(18 + 17) }"), "35");
}

#[test]
fn compile_run_batch_1868() {
    assert_eq!(run("func main() { print(19 + 18) }"), "37");
}

#[test]
fn compile_run_batch_1869() {
    assert_eq!(run("func main() { print(20 + 19) }"), "39");
}

#[test]
fn compile_run_batch_1870() {
    assert_eq!(run("func main() { print(21 + 20) }"), "41");
}

#[test]
fn compile_run_batch_1871() {
    assert_eq!(run("func main() { print(22 + 21) }"), "43");
}

#[test]
fn compile_run_batch_1872() {
    assert_eq!(run("func main() { print(23 + 22) }"), "45");
}

#[test]
fn compile_run_batch_1873() {
    assert_eq!(run("func main() { print(24 + 23) }"), "47");
}

#[test]
fn compile_run_batch_1874() {
    assert_eq!(run("func main() { print(25 + 24) }"), "49");
}

#[test]
fn compile_run_batch_1875() {
    assert_eq!(run("func main() { print(26 + 25) }"), "51");
}

#[test]
fn compile_run_batch_1876() {
    assert_eq!(run("func main() { print(27 + 26) }"), "53");
}

#[test]
fn compile_run_batch_1877() {
    assert_eq!(run("func main() { print(28 + 27) }"), "55");
}

#[test]
fn compile_run_batch_1878() {
    assert_eq!(run("func main() { print(29 + 28) }"), "57");
}

#[test]
fn compile_run_batch_1879() {
    assert_eq!(run("func main() { print(30 + 29) }"), "59");
}

#[test]
fn compile_run_batch_1880() {
    assert_eq!(run("func main() { print(31 + 30) }"), "61");
}

#[test]
fn compile_run_batch_1881() {
    assert_eq!(run("func main() { print(32 + 31) }"), "63");
}

#[test]
fn compile_run_batch_1882() {
    assert_eq!(run("func main() { print(33 + 32) }"), "65");
}

#[test]
fn compile_run_batch_1883() {
    assert_eq!(run("func main() { print(34 + 33) }"), "67");
}

#[test]
fn compile_run_batch_1884() {
    assert_eq!(run("func main() { print(35 + 34) }"), "69");
}

#[test]
fn compile_run_batch_1885() {
    assert_eq!(run("func main() { print(36 + 35) }"), "71");
}

#[test]
fn compile_run_batch_1886() {
    assert_eq!(run("func main() { print(37 + 36) }"), "73");
}

#[test]
fn compile_run_batch_1887() {
    assert_eq!(run("func main() { print(38 + 37) }"), "75");
}

#[test]
fn compile_run_batch_1888() {
    assert_eq!(run("func main() { print(39 + 38) }"), "77");
}

#[test]
fn compile_run_batch_1889() {
    assert_eq!(run("func main() { print(40 + 39) }"), "79");
}

#[test]
fn compile_run_batch_1890() {
    assert_eq!(run("func main() { print(41 + 40) }"), "81");
}

#[test]
fn compile_run_batch_1891() {
    assert_eq!(run("func main() { print(42 + 41) }"), "83");
}

#[test]
fn compile_run_batch_1892() {
    assert_eq!(run("func main() { print(43 + 42) }"), "85");
}

#[test]
fn compile_run_batch_1893() {
    assert_eq!(run("func main() { print(44 + 43) }"), "87");
}

#[test]
fn compile_run_batch_1894() {
    assert_eq!(run("func main() { print(45 + 44) }"), "89");
}

#[test]
fn compile_run_batch_1895() {
    assert_eq!(run("func main() { print(46 + 45) }"), "91");
}

#[test]
fn compile_run_batch_1896() {
    assert_eq!(run("func main() { print(47 + 46) }"), "93");
}

#[test]
fn compile_run_batch_1897() {
    assert_eq!(run("func main() { print(48 + 47) }"), "95");
}

#[test]
fn compile_run_batch_1898() {
    assert_eq!(run("func main() { print(49 + 48) }"), "97");
}

#[test]
fn compile_run_batch_1899() {
    assert_eq!(run("func main() { print(50 + 49) }"), "99");
}

#[test]
fn compile_run_batch_1900() {
    assert_eq!(run("func main() { print(51 + 50) }"), "101");
}

#[test]
fn compile_run_batch_1901() {
    assert_eq!(run("func main() { print(52 + 51) }"), "103");
}

#[test]
fn compile_run_batch_1902() {
    assert_eq!(run("func main() { print(53 + 52) }"), "105");
}

#[test]
fn compile_run_batch_1903() {
    assert_eq!(run("func main() { print(54 + 53) }"), "107");
}

#[test]
fn compile_run_batch_1904() {
    assert_eq!(run("func main() { print(55 + 54) }"), "109");
}

#[test]
fn compile_run_batch_1905() {
    assert_eq!(run("func main() { print(56 + 55) }"), "111");
}

#[test]
fn compile_run_batch_1906() {
    assert_eq!(run("func main() { print(57 + 56) }"), "113");
}

#[test]
fn compile_run_batch_1907() {
    assert_eq!(run("func main() { print(58 + 57) }"), "115");
}

#[test]
fn compile_run_batch_1908() {
    assert_eq!(run("func main() { print(59 + 58) }"), "117");
}

#[test]
fn compile_run_batch_1909() {
    assert_eq!(run("func main() { print(60 + 59) }"), "119");
}

#[test]
fn compile_run_batch_1910() {
    assert_eq!(run("func main() { print(61 + 60) }"), "121");
}

#[test]
fn compile_run_batch_1911() {
    assert_eq!(run("func main() { print(62 + 61) }"), "123");
}

#[test]
fn compile_run_batch_1912() {
    assert_eq!(run("func main() { print(63 + 62) }"), "125");
}

#[test]
fn compile_run_batch_1913() {
    assert_eq!(run("func main() { print(64 + 63) }"), "127");
}

#[test]
fn compile_run_batch_1914() {
    assert_eq!(run("func main() { print(65 + 64) }"), "129");
}

#[test]
fn compile_run_batch_1915() {
    assert_eq!(run("func main() { print(66 + 65) }"), "131");
}

#[test]
fn compile_run_batch_1916() {
    assert_eq!(run("func main() { print(67 + 66) }"), "133");
}

#[test]
fn compile_run_batch_1917() {
    assert_eq!(run("func main() { print(68 + 67) }"), "135");
}

#[test]
fn compile_run_batch_1918() {
    assert_eq!(run("func main() { print(69 + 68) }"), "137");
}

#[test]
fn compile_run_batch_1919() {
    assert_eq!(run("func main() { print(70 + 69) }"), "139");
}

