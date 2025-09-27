#!/bin/bash

check_if_its_root() {
    if [ "$(id -u)" -ne 0 ]; then
        echo "You must run this script as root.";
        exit 1;
    fi
}

install_binary_files() {
    echo "Installing binary files...";

    if [ -f septblog-backend/target/release/septblog-backend ]; then
        cp -v septblog-backend/target/release/septblog-backend /usr/local/bin/septblog-backend;
    else
        echo "Missing septblog-backend/target/release/septblog-backend file...";
        echo "Make sure you build the project at first...";
        echo "Installation aborted...";
        exit 456;
    fi

    if [ -f septblog-frontend/target/release/septblog-frontend ]; then
        cp -v septblog-frontend/target/release/septblog-frontend /usr/local/bin/septblog-frontend;
    else
        echo "Missing septblog-frontend/target/release/septblog-frontend file...";
        echo "Make sure you build the project at first...";
        echo "Installation aborted...";
        exit 893;
    fi
}

create_etc_septblog_if_not_exist() {
    if [ ! -d /etc/septblog ]; then
        mkdir /etc/septblog;
        chmod 750 /etc/septblog;
        chown -v root:septblog /etc/septblog;
    fi
}

copy_config_file() {
    if [ -f septblog-backend/src/config.json ]; then
        if [ ! -f /etc/septblog/backend.json ]; then
            cp -v septblog-backend/src/config.json /etc/septblog/backend.json;
            chmod 640 /etc/septblog/backend.json;
            chown -v root:septblog /etc/septblog/backend.json;
        fi
    else
        echo "Missing septblog-backend/src/config.json file...";
        echo "Installation aborted...";

        exit 235;
    fi

    if [ -f septblog-frontend/src/config.json ]; then
        if [ ! -f /etc/septblog/frontend.json ]; then
            cp -v septblog-frontend/src/config.json /etc/septblog/frontend.json;
            chmod 640 /etc/septblog/frontend.json;
            chown -v root:septblog /etc/septblog/frontend.json;
        fi
    else
        echo "Missing septblog-frontend/src/config.json file...";
        echo "Installation aborted...";

        exit 236;
    fi
}

create_user() {
    adduser --system --no-create-home --group septblog;
}

create_systemd_service_file_for_backend() {
    cat <<EOF > /etc/systemd/system/septblog-backend.service
[Unit]
Description=SeptBlog Backend Service
After=network.target

[Service]
User=septblog
Group=septblog

ExecStart=/usr/local/bin/septblog-backend

Restart=on-failure
RestartSec=120

Environment="RUST_LOG=error"

WorkingDirectory=/var/www/septblog

[Install]
WantedBy=multi-user.target
EOF
}

create_systemd_service_file_for_frontend() {
    cat <<EOF > /etc/systemd/system/septblog-frontend.service
[Unit]
Description=SeptBlog Frontend Service
After=network.target

[Service]
User=septblog
Group=septblog

ExecStart=/usr/local/bin/septblog-frontend

Restart=on-failure
RestartSec=120

Environment="RUST_LOG=error"

WorkingDirectory=/var/www/septblog

[Install]
WantedBy=multi-user.target
EOF
}

reload_systemd() {
    systemctl daemon-reload;
}

if_var_www_is_not_exist() {
    if [ ! -d /var/www ]; then
        mkdir -p /var/www;
        chown -v root:root /var/www;
    fi
}

create_var_www_septblog_dir() {
    if [ ! -d /var/www/septblog ]; then
        mkdir -p /var/www/septblog;
    fi

    chown -Rv septblog:septblog /var/www/septblog;
}

create_var_www_septblog_public_dir() {
    if [ ! -d /var/www/septblog/public ]; then
        mkdir -p /var/www/septblog/public;
    fi

    chown -Rv septblog:septblog /var/www/septblog/public;
}

create_var_www_septblog_uploads_dir() {
    if [ ! -d /var/www/septblog/uploads ]; then
        mkdir -p /var/www/septblog/uploads;
    fi

    chown -v septblog:septblog /var/www/septblog/uploads;
}

cp_profile_photo_image_file() {
    if [ -f septblog-backend/src/user.jpg ]; then
        cp -v septblog-backend/src/user.jpg /var/www/septblog/uploads;
        chown -v septblog:septblog /var/www/septblog/uploads/user.jpg
    fi
}

install_frontend() {
    cp -v septblog-frontend/src/html/*.html /var/www/septblog/public/;
    cp -Rv septblog-frontend/src/html/admin/ /var/www/septblog/public/;
    cp -Rv septblog-frontend/src/html/assets/ /var/www/septblog/public/;
    cp -Rv septblog-frontend/src/html/css/ /var/www/septblog/public/;
    cp -Rv septblog-frontend/src/html/js/ /var/www/septblog/public/;

    chown -Rv septblog:septblog /var/www/septblog/public/
}

fire() {
    check_if_its_root;
    install_binary_files;
    create_etc_septblog_if_not_exist;
    copy_config_file;
    create_user;
    create_systemd_service_file_for_frontend;
    create_systemd_service_file_for_backend;
    reload_systemd;
    if_var_www_is_not_exist;
    create_var_www_septblog_dir;
    create_var_www_septblog_public_dir;
    create_var_www_septblog_uploads_dir;
    cp_profile_photo_image_file;
    install_frontend;
}

fire;