CREATE TABLE IF NOT EXISTS vehicles (
    id VARCHAR(36) PRIMARY KEY,
    vin VARCHAR(17) UNIQUE NOT NULL,
    soc TINYINT,
    target_soc TINYINT NOT NULL,
    last_update_time DATETIME(3),

    INDEX idx_vin (vin)
);

CREATE TABLE IF NOT EXISTS charge_sessions (
    id VARCHAR(36) PRIMARY KEY,
    start_time DATETIME(3) NOT NULL,
    stop_time DATETIME(3),
    start_soc TINYINT NOT NULL,
    current_soc TINYINT,
    stop_soc TINYINT,
    vehicle_id VARCHAR(36) NOT NULL,
    last_update_time DATETIME(3),

    FOREIGN KEY (vehicle_id) REFERENCES vehicles (id),
    INDEX idx_start_time (start_time),
    INDEX idx_stop_time (stop_time)
);

CREATE TABLE IF NOT EXISTS users (
    id VARCHAR(36) PRIMARY KEY,
    username VARCHAR(255) UNIQUE NOT NULL,
    password_hash VARCHAR(255) NOT NULL,
    creation_time DATETIME(3) NOT NULL,

    INDEX idx_username (username)
);
