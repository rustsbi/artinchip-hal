//! QSPI pin mux for D13x series.

// QSPI0
qspi_sck!(0, ('B', 4, 2));
qspi_mosi!(0, ('B', 5, 2));
qspi_miso!(0, ('B', 1, 2));
qspi_cs!(0, ('B', 2, 2));
qspi_hold!(0, ('B', 3, 2));
qspi_wp!(0, ('B', 0, 2));

// QSPI1
qspi_sck!(1, ('B', 4, 3), ('D', 7, 3));
qspi_mosi!(1, ('B', 5, 3), ('D', 6, 3));
qspi_miso!(1, ('B', 1, 3), ('D', 5, 3));
qspi_cs!(1, ('B', 2, 3), ('D', 4, 3));
qspi_hold!(1, ('B', 3, 3), ('D', 8, 3));
qspi_wp!(1, ('B', 0, 3), ('D', 9, 3));

// QSPI2
qspi_sck!(2, ('E', 12, 2), ('B', 9, 3));
qspi_mosi!(2, ('E', 14, 2), ('B', 8, 3));
qspi_miso!(2, ('E', 15, 2), ('B', 7, 3));
qspi_cs!(2, ('E', 13, 2), ('B', 6, 3));
qspi_hold!(2, ('E', 16, 2), ('B', 10, 3));
qspi_wp!(2, ('E', 17, 2), ('B', 11, 3));

// QSPI3
qspi_sck!(3, ('C', 8, 2), ('D', 0, 3));
qspi_mosi!(3, ('C', 10, 2), ('D', 2, 3));
qspi_miso!(3, ('C', 11, 2), ('D', 3, 3));
qspi_cs!(3, ('C', 9, 2), ('D', 1, 3));
