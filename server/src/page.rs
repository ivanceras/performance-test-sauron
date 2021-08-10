use client::{App, Msg};
use sauron::prelude::*;

/// We are creating an index page.
/// From the `App` supplied, we can derive the view by calling `App.view` function.
/// we extract the state and serialize it.
pub fn index() -> Node<Msg> {
    node! {
        <html lang="en" ng-app="test">

        <head>
            <title>"Performance Comparison for Sauron, Angular and React"</title>
            <link href="//cdnjs.cloudflare.com/ajax/libs/twitter-bootstrap/3.3.1/css/bootstrap.css" rel="stylesheet" />
            <style type="text/css">
                {text(r#"
                * { box-sizing:border-box; }
                                        body { padding:30px 0; }
                                        h2 { margin:0; margin-bottom:25px; }
                                        h3 { margin:0; padding:0; margin-bottom:12px; }
                                        .test-data { margin-bottom:3px; }
                                        .test-data span { padding:3px 10px; background:#EEE; width:100%; float:left; cursor:pointer; }
                                        .test-data span:hover { background:#DDD; }
                                        .test-data span.selected { background:#3F7AD9; color:white; }

                                        .time { font-weight:bold; height:26px; line-height:26px; vertical-align:middle; display:inline-block; cursor:pointer; text-decoration:underline; }
                "#)}
            </style>

            <script type="text/javascript" src="//cdnjs.cloudflare.com/ajax/libs/angular.js/1.3.3/angular.min.js"></script>
            <script type="text/javascript" src="//cdnjs.cloudflare.com/ajax/libs/react/0.12.1/react.js"></script>
            <script type="text/javascript">
                {text(r#"
                        console.timeEnd("build");

                        document.addEventListener("DOMContentLoaded", function () {
                            _react();
                            _raw();
                        });

                        _angular();

                        function _buildData(count) {
                            count = count || 1000;

                            var adjectives = [
                                "pretty",
                                "large",
                                "big",
                                "small",
                                "tall",
                                "short",
                                "long",
                                "handsome",
                                "plain",
                                "quaint",
                                "clean",
                                "elegant",
                                "easy",
                                "angry",
                                "crazy",
                                "helpful",
                                "mushy",
                                "odd",
                                "unsightly",
                                "adorable",
                                "important",
                                "inexpensive",
                                "cheap",
                                "expensive",
                                "fancy",
                            ];
                            var colours = ["red", "yellow", "blue", "green", "pink", "brown", "purple", "brown", "white", "black", "orange"];
                            var nouns = ["table", "chair", "house", "bbq", "desk", "car", "pony", "cookie", "sandwich", "burger", "pizza", "mouse", "keyboard"];
                            var data = [];
                            for (var i = 0; i < count; i++) data.push({ id: i + 1, label: adjectives[_random(adjectives.length)] + " " + colours[_random(colours.length)] + " " + nouns[_random(nouns.length)] });
                            return data;
                        }

                        function _random(max) {
                            return Math.round(Math.random() * 1000) % max;
                        }


                        function _angular(data) {
                            angular.module("test", []).controller("controller", function ($scope) {
                                $scope.run = function () {
                                    var data = _buildData(),
                                        date = performance.now();

                                    $scope.selected = null;
                                    $scope.$$postDigest(function () {
                                        document.getElementById("run-angular").innerHTML = Math.round(performance.now() - date) + " ms";
                                    });

                                    $scope.data = data;
                                };

                                $scope.select = function (item) {
                                    $scope.selected = item.id;
                                };
                            });
                        }

                        function _react() {
                            var Class = React.createClass({
                                select: function (data) {
                                    this.props.selected = data.id;
                                    this.forceUpdate();
                                },

                                render: function () {
                                    var items = [];
                                    for (var i = 0; i < this.props.data.length; i++) {
                                        items.push(
                                            React.createElement(
                                                "div",
                                                { className: "row" },
                                                React.createElement(
                                                    "div",
                                                    { className: "col-md-12 test-data" },
                                                    React.createElement("span", { className: this.props.selected === this.props.data[i].id ? "selected" : "", onClick: this.select.bind(null, this.props.data[i]) }, this.props.data[i].label)
                                                )
                                            )
                                        );
                                    }

                                    return React.createElement("div", null, items);
                                },
                            });

                            var runReact = document.getElementById("run-react");
                            runReact.addEventListener("click", function () {
                                var data = _buildData(),
                                    date = performance.now();

                                React.render(new Class({ data: data, selected: null }), document.getElementById("react"));
                                runReact.innerHTML = Math.round(performance.now() - date) + " ms";
                            });
                        }

                        function _raw() {
                            var container = document.getElementById("raw"),
                                docFragment = document.createDocumentFragment(),
                                runRawNode = document.getElementById("run-raw"),
                                handler = function () {
                                    var selected = container.querySelector(".selected");
                                    if (selected) {
                                        selected.className = "";
                                    }
                                    this.className = "selected";
                                };
                            runRawNode.addEventListener("click", function () {
                                var data = _buildData(),
                                    date = performance.now();

                                if (!container.hasChildNodes()) {
                                    var containerWrap = document.createElement("div");
                                    container.appendChild(containerWrap);
                                    for (var i = 0; i < data.length; i++) {
                                        var div1 = document.createElement("div"),
                                            div2 = document.createElement("div"),
                                            span = document.createElement("span");
                                        div1.className = "row";
                                        div2.className = "col-md-12 test-data";
                                        span.addEventListener("click", handler);
                                        span.textContent = data[i].label;
                                        div2.appendChild(span);
                                        div1.appendChild(div2);
                                        docFragment.appendChild(div1);
                                    }
                                    containerWrap.appendChild(docFragment);
                                    container.appendChild(containerWrap);
                                } else {
                                    for (var i = 0; i < data.length; i++) {
                                        container.firstChild.childNodes[i].firstChild.firstChild.className = "";
                                        container.firstChild.childNodes[i].firstChild.firstChild.textContent = data[i].label;
                                    }
                                }
                                runRawNode.textContent = Math.round(performance.now() - date) + " ms";
                            });
                        }
                    "#)}
            </script>
            <script type="module">
                {text!("
                     import init from '/pkg/client.js';
                        init().catch(console.error);
                    ")}
            </script>
        </head>

        <body ng-controller="controller">
            <div class="container">
                <div class="row">
                    <div class="col-md-12">
                        <h2>"Performance Comparison for React, Angular and Sauron"</h2>
                    </div>
                </div>

                <div class="col-md-3">
                    <div class="row">
                        <div class="col-md-7">
                            <h3>"React"</h3>
                        </div>
                        <div class="col-md-5 text-right time" id="run-react">"Run"</div>
                    </div>
                    <div id="react"></div>
                </div>

                <div class="col-md-3">
                    <div class="row">
                        <div class="col-md-7">
                            <h3>"Angular"</h3>
                        </div>
                        <div class="col-md-5 text-right time" id="run-angular" ng-click="run()">"Run"</div>
                    </div>
                    <div>
                        <div class="row" ng-repeat="item in data">
                            <div class="col-md-12 test-data">
                                <span ng-class="{ selected: item.id === $parent.selected }" ng-click="select(item)">{text("{{item.label}}")}</span>
                            </div>
                        </div>
                    </div>
                </div>

                <div class="col-md-3">
                    <div class="row">
                        <div class="col-md-7">
                            <h3>"Raw"</h3>
                        </div>
                        <div class="col-md-5 text-right time" id="run-raw">"Run"</div>
                    </div>
                    <div id="raw"></div>
                </div>

                <div class="col-md-3">
                    <div class="row">
                        <div class="col-md-7">
                            <h3>"Sauron"</h3>
                        </div>
                        <div class="col-md-5 text-right time" id="run-sauron">"Run"</div>
                    </div>
                    <div id="sauron"></div>
                </div>
            </div>

            <script type="text/html" id="raw-template">
                <div class="row">
                    <div class="col-md-12 test-data">
                        <span class="{{className}}">{text("{{label}}")}</span>
                    </div>
                </div>
            </script>
        </body>

        </html>
    }
}

/// This could just be serve as a style.css or just a static string
/// jss can be used to allow you to dynamically create server side themes by manipulating the css
/// here.
fn style() -> String {
    jss!({
        "body": {
          "font-family": "'Avenir Next', 'Helvetica Neue', 'Segoe UI', 'Helvetica', 'Arial', 'sans-serif'",
          "margin": 0,
          "padding": 0,
        },

        "header, footer": {
          "background-color": "#FCB799",
          "display": "flex",
          "flex-direction": "row",
          "justify-content": "space-between",
          "margin": 0,
          "padding": "1em",
          "width": "calc(100vw - 2em)",
        },

        "footer": {
          "flex-direction": "row-reverse",
          "position": "fixed",
          "bottom": 0,
        },

        "header > section": {
          "text-align": "right",
        },

        "header > svg": {
          "width": "4em",
          "height": "auto",
        },

        "header a": {
          "display": "flex",
          "flex-direction": "column",
          "align-items": "center",
          "justify-content": "center",
        },

        "h1": {
          "margin": 0,
          "padding": 0,
          "font-size": "1.5em",
        },

        "h2": {
          "margin": 0,
          "padding": 0,
          "font-size": "1em",
          "font-style": "italic",
        },

        ".visually-hidden": {
          "clip": "rect(0 0 0 0)",
          "clip-path": "inset(50%)",
          "height": "1px",
          "overflow": "hidden",
          "position": "absolute",
          "white-space": "nowrap",
          "width": "1px",
        },

        "main": {
          "padding": "2em",
          "display": "flex",
          "flex-direction": "column",
          "align-items": "center",
          "justify-content": "center",
          "text-align": "center",
        },

        "form, form > label": {
          "display": "flex",
          "flex-direction": "column",
          "font-weight": "bold",
          "justify-content": "center",
          "align-items": "center",
        },

        "input, button": {
          "margin": "0.25em",
        },

        ".modified-name, .length": {
          "font-weight": "bold",
          "color": "rgb(247, 76, 0)",
        },
    })
}
