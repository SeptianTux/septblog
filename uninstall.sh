#!/bin/bash

check_if_its_root() {
    if [ "$(id -u)" -ne 0 ]; then
        echo "You must run this script as root.";
        exit 1;
    fi
}

uninstall_frontend() {
    if [ -d /var/www/septblog ]; then
        rm -Rv /var/www/septblog;
    fi
}

stop_septblog_services() {
    if systemctl is-active --quiet septblog-backend.service; then
        systemctl stop septblog-backend.service;
    fi

    if systemctl is-active --quiet septblog-frontend.service; then
        systemctl stop septblog-frontend.service;
    fi
}

delete_systemd_service_file_for_frontend() {
    if [ -f /etc/systemd/system/septblog-frontend.service ]; then
        rm -v /etc/systemd/system/septblog-frontend.service;
    fi
}

delete_systemd_service_file_for_backend() {
    if [ -f /etc/systemd/system/septblog-backend.service ]; then
        rm -v /etc/systemd/system/septblog-backend.service;
    fi
}

reload_systemd() {
    systemctl daemon-reload;
}

delete_septblog_user() {
    if grep -q "^septblog:" /etc/passwd; then
        userdel -f septblog;
    fi
}

uninstall_binary_files() {
    if [ -f /usr/local/bin/septblog-backend ]; then
        rm -v /usr/local/bin/septblog-backend;
    fi

    if [ -f /usr/local/bin/septblog-frontend ]; then
        rm -v /usr/local/bin/septblog-frontend;
    fi
}

fire() {
    check_if_its_root;
    uninstall_frontend;
    stop_septblog_services;
    delete_systemd_service_file_for_frontend;
    delete_systemd_service_file_for_backend;
    delete_septblog_user;
    uninstall_binary_files;
}

fire;