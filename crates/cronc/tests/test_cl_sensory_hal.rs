use cronc::cl_sensory_hal::*;

#[test]
fn test_dvs_event_packing_and_spatial_mapping() {
    let event = DvsEvent::new(256, 128, 1, 54321);
    let packet = event.to_u32_packet();
    let unpacked = DvsEvent::from_u32_packet(packet, 54000);

    assert_eq!(unpacked.x, 256);
    assert_eq!(unpacked.y, 128);
    assert_eq!(unpacked.polarity, 1);
    assert_eq!(unpacked.timestamp_us, 54000 + (54321 & 0x1FFF));

    // Map to 4D Torus coordinates (sensor resolution 512x512)
    let coord = event.map_to_torus_coord(512, 512);
    assert!(coord.x < 4);
    assert!(coord.y < 4);
    assert!(coord.z < 4);
    assert!(coord.w < 4);
    assert_eq!(coord.x, 2);
    assert_eq!(coord.y, 1);
}

#[test]
fn test_cochlea_tonotopic_mapping() {
    let spike = CochleaSpike::new(42, 300, 1000);
    let coord = spike.map_to_torus_coord();

    assert!(coord.x < 4);
    assert!(coord.y < 4);
    assert!(coord.z < 4);
    assert!(coord.w < 4);
    assert_eq!(coord.x, 42 % 4);
    assert_eq!(coord.y, (42 / 4) % 4);
    assert_eq!(coord.z, (42 / 16) % 4);
    assert_eq!(coord.w, (42 / 64) % 4);
}

#[test]
fn test_time_surface_accumulator_and_patch_extraction() {
    let mut accumulator = TimeSurfaceAccumulator::new(64, 64, 5000.0);

    let event1 = DvsEvent::new(32, 32, 1, 1000);
    let salience1 = accumulator.update(&event1);
    assert_eq!(salience1, 1.0);

    let event2 = DvsEvent::new(32, 32, 1, 2000);
    let salience2 = accumulator.update(&event2);
    assert!(salience2 > 0.8 && salience2 < 1.0, "Exponential decay expected: {}", salience2);

    let patch = accumulator.extract_patch_64(32, 32);
    assert_eq!(patch.len(), 64);
}
