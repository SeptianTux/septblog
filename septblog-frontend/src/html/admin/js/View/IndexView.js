'use strict';

import { UnauthorizedError } from "../Errors/UnauthorizedError.js";
import { IndexViewModel } from "../ViewModel/IndexViewModel.js";
import { Page } from "../Utils/Page.js";
import { User } from "../Utils/User.js";

export class IndexView {
    #config = null;
    viewModel = null;

    constructor(config) {
        this.#config = config;
        this.viewModel = new IndexViewModel(this.#config);
        this.#visitorsChart();
        this.#visitorsTable();

        this.#setLoggedInAs();
        this.#setPageTitle();
        this.#setNavbarBrand();
        this.#setCopyright();

        this.#administratorsNavMenu();
    }

    async #administratorsNavMenu() {
        await User.administratorsNavMenu();
    }

    async #setCopyright() {
        await Page.setCopyright();
    }

    async #setPageTitle() {
        await Page.setSiteTitleAdmin();
    }

    async #setNavbarBrand() {
        await Page.setNavbarBrand();
    }

    async #setLoggedInAs() {
        await Page.setLoggedInAs();
    }

    toUnixTimestamp(dateStr) {
        // Expecting format "dd/mm/yyyy"
        const [day, month, year] = dateStr.split("/").map(Number);

        // JavaScript Date uses month index starting at 0 (0 = January)
        const date = new Date(year, month - 1, day);

        // Convert milliseconds → seconds
        return Math.floor(date.getTime() / 1000);
    }

    getMonthName(monthNumber) {
        const months = [
            "January", "February", "March", "April", "May", "June",
            "July", "August", "September", "October", "November", "December"
        ];

        // monthNumber is 1–12
        return months[monthNumber - 1] || null;
    }

    getChartVisitorsTimestamp(monthMinus) {
        const date = new Date();
        let month = date.getMonth() - monthMinus;
        let year = date.getFullYear();

        if (monthMinus >= 11) {
            return 0;
        }

        if (month <= 0 ) {
            year--;
            month = 11 - monthMinus;
        }

        const epochTime = this.toUnixTimestamp("1/" + (month + 1) + "/" + year);

        return epochTime;
    }

    async getVisitorForChart(start, end) {
        let res = null;

        try {
            res = await this.viewModel.getVisitorForChart(start, end);
        } catch(error) {
            if (error instanceof UnauthorizedError) {
                this.#redirectToLoginPage();
            } else if (error instanceof TypeError) {
                this.alertDanger("Failed to connect to network.");
            } else {
                this.alertDanger(error.message);
            }
        }

        return res.data.visitors
    }

    async getVisitorForTable() {
        let res = null;

        try {
            res = await this.viewModel.getVisitorForTable();
        } catch(error) {
            if (error instanceof UnauthorizedError) {
                this.#redirectToLoginPage();
            } else if (error instanceof TypeError) {
                this.alertDanger("Failed to connect to network.");
            } else {
                this.alertDanger(error.message);
            }
        }

        return res.data;
    }

    async visitorsChartData() {
        let ret = new Array();
        let monthNow = new Date().getMonth();

        for (let i = 0; i <= 5; i++) {
            ret[i] = new Array();
        }

        ret[0]["month"] = this.getMonthName(new Date().getMonth() + 1);
        ret[0]["visitors"] = await this.getVisitorForChart(
            this.toUnixTimestamp("1/" + (new Date().getMonth() + 1) + "/" + new Date().getFullYear()),
            Math.floor(Date.now() / 1000)
        );

        let monthMinusStart = 1;
        let monthMinusEnd = 0;

        for (let i = 1; i < ret.length; i++) {
            ret[i]["month"] = this.getMonthName(monthNow);

            const start = this.getChartVisitorsTimestamp(monthMinusStart);
            const end = this.getChartVisitorsTimestamp(monthMinusEnd);

            ret[i]["visitors"] = await this.getVisitorForChart(start, end);

            monthNow--;
            monthMinusStart++;
            monthMinusEnd++;
        }

        return ret;
    }

    async #visitorsChart() {
        const ctx = document.getElementById('visitorsChart');
        const visitorsData = await this.visitorsChartData();

        new Chart(ctx, {
                type: 'line',
                data: {
                    labels: [
                                visitorsData[5].month,
                                visitorsData[4].month,
                                visitorsData[3].month,
                                visitorsData[2].month,
                                visitorsData[1].month,
                                visitorsData[0].month
                            ],
                    datasets: [{
                        label: '# of Visitors',
                        data: [
                                visitorsData[5].visitors,
                                visitorsData[4].visitors,
                                visitorsData[3].visitors,
                                visitorsData[2].visitors,
                                visitorsData[1].visitors,
                                visitorsData[0].visitors,
                            ],
                        borderWidth: 1
                    }]
                },
                options: {
                    scales: {
                        y: {
                            beginAtZero: true,
                            ticks: {
                                precision: 0
                            },
                        }
                    }
                }
            });
    }

    copyToClipboard(value) {
        navigator.clipboard.writeText(value);
        /*
        .then(() => {
            alert("Copied to clipboard: " + text);
        })
        .catch(err => {
            console.error("Failed to copy: ", err);
        });
        */
    }

    copyUserAgentToClipboard(id) {
        const userAgent = document.getElementById(id).value;
        
        this.copyToClipboard(userAgent);
    }

    copyRefererToClipboard(id) {
        const referer = document.getElementById(id).value;

        this.copyToClipboard(referer);
    }

    async #visitorsTable() {
        const visitorData = await this.getVisitorForTable();

        if (visitorData.length > 0) {
            const visitorsTable =   '<div class="table-responsive">' +
                                        '<table id="tableVisitors" class="table table-striped table-hover table-bordered table-visitors">' +
                                            '<thead>' +
                                                '<tr>' +
                                                    '<th>Article</th>' +
                                                    '<th>User Agent</th>' +
                                                    '<th>IP Adress</th>' +
                                                    '<th>Referer</th>' +
                                                    '<th>Date/Time</th>' +
                                                '</tr>' +
                                            '</thead>' +
                                            '<tfoot>' +
                                                '<tr>' +
                                                    '<th>Article</th>' +
                                                    '<th>User Agent</th>' +
                                                    '<th>IP Adress</th>' +
                                                    '<th>Referer</th>' +
                                                    '<th>Date/Time</th>' +
                                                '</tr>' +
                                            '</tfoot>' +
                                            '<tbody id="visitorsTableData">' +
                                                '<tr>' +
                                                    '<td>' +
                                                        '<div class="d-flex justify-content-center">' +
                                                            '<div class="spinner-border" role="status">' +
                                                                '<span class="visually-hidden">Loading...</span>' +
                                                            '</div>' +
                                                        '</div>' +
                                                    '</td>' +
                                                    '<td>' +
                                                        '<div class="d-flex justify-content-center">' +
                                                            '<div class="spinner-border" role="status">' +
                                                                '<span class="visually-hidden">Loading...</span>' +
                                                            '</div>' +
                                                        '</div>' +
                                                    '</td>' +
                                                    '<td>' +
                                                        '<div class="d-flex justify-content-center">' +
                                                            '<div class="spinner-border" role="status">' +
                                                                '<span class="visually-hidden">Loading...</span>' +
                                                            '</div>' +
                                                        '</div>' +
                                                    '</td>' +
                                                    '<td>' +
                                                        '<div class="d-flex justify-content-center">' +
                                                            '<div class="spinner-border" role="status">' +
                                                                '<span class="visually-hidden">Loading...</span>' +
                                                            '</div>' +
                                                        '</div>' +
                                                    '</td>' +
                                                    '<td>' +
                                                        '<div class="d-flex justify-content-center">' +
                                                            '<div class="spinner-border" role="status">' +
                                                                '<span class="visually-hidden">Loading...</span>' +
                                                            '</div>' +
                                                        '</div>' +
                                                    '</td>' +
                                                '</tr>' +
                                            '</tbody>' +
                                        '</table>' +
                                    '</div>';

            document.getElementById("cardBody").insertAdjacentHTML("beforeend", visitorsTable);
            document.getElementById("visitorsTableData").replaceChildren();

            for (let i = 0; i < visitorData.length; i++) {
                const date = new Date(visitorData[i].visited_at * 1000);
                const tr = '<tr>' +
                                '<td>' +
                                    '<a href="/article/' + visitorData[i].article_id + '">' + visitorData[i].article_title + '</a>' +
                                '</td>' +
                                '<td>' +
                                    '<div class="input-group mb-3">' +
                                        '<input id="userAgent-' + visitorData[i].article_id + '" type="text" class="form-control" placeholder="User Agent" aria-label="User Agent" aria-describedby="button-addon2" value="' + visitorData[i].user_agent + '">' +
                                        '<button onClick="main.view.copyUserAgentToClipboard(\'userAgent-' + visitorData[i].article_id + '\');" class="btn btn-outline-secondary" type="button" id="button-addon2"><i class="fa-solid fa-clipboard"></i></button>' +
                                    '</div>' +
                                '</td>' +
                                '<td>' +
                                    visitorData[i].ip_address +
                                '</td>' +
                                '<td>' +
                                    '<div class="input-group mb-3">' +
                                        '<input id="referer-' + visitorData[i].article_id + '" type="text" class="form-control" placeholder="Referer" aria-label="User Agent" aria-describedby="button-addon2" value="' + visitorData[i].referer + '">' +
                                        '<button onClick="main.view.copyRefererToClipboard(\'referer-' + visitorData[i].article_id + '\');" class="btn btn-outline-secondary" type="button" id="button-addon2"><i class="fa-solid fa-clipboard"></i></button>' +
                                    '</div>' +
                                '</td>' +
                                '<td>' +
                                    date.getDate() + '/' + (date.getMonth() + 1) + '/' + date.getFullYear() + ' ' + date.getHours() + ':' + date.getMinutes() + ':' + date.getSeconds() +
                                '</td>' +
                            '</tr>';
                
                document.getElementById("visitorsTableData").insertAdjacentHTML("beforeend", tr);
            }
        } else {
            const noVisitorsWarning = '<div class="dashboard-no-visitors-warning text-muted">There are no visitors yet.</div>';
            document.getElementById("cardBody").insertAdjacentHTML("beforeend", noVisitorsWarning);
        }
    }

    #redirectToLoginPage() {
        const path = "/admin";
        const redirectTo = encodeURIComponent(path);

        window.location.href = "/admin/login?redirect-to=" + redirectTo;
    }

    alertDanger(message) {
        const alert = '<div id="alert" class="alert alert-danger alert-dismissible fade show" role="alert">' +
                            message +
                            '<button type="button" class="btn-close" data-bs-dismiss="alert" aria-label="Close"></button>' +
                        '</div>';
        
        document.getElementById("alertContainer").insertAdjacentHTML("beforeend", alert);
    }

    alertSuccess(message) {
        const alert = '<div id="alert" class="alert alert-success alert-dismissible fade show" role="alert">' +
                            message +
                            '<button type="button" class="btn-close" data-bs-dismiss="alert" aria-label="Close"></button>' +
                        '</div>';
        
        document.getElementById("alertContainer").insertAdjacentHTML("beforeend", alert);
    }

    alertClose() {
        const alertElement = document.getElementById('alert');

        if (alertElement !== null) {
            const alertInstance = bootstrap.Alert.getOrCreateInstance(alertElement);

            alertInstance.close();
        }
    }
}