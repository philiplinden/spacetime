<!DOCTYPE html>

<html lang="en">
<head><meta charset="utf-8"/>
<meta content="width=device-width, initial-scale=1.0" name="viewport"/>
<title>networkx_learning</title><script src="https://cdnjs.cloudflare.com/ajax/libs/require.js/2.1.10/require.min.js"></script>
<style type="text/css">
    pre { line-height: 125%; }
td.linenos .normal { color: inherit; background-color: transparent; padding-left: 5px; padding-right: 5px; }
span.linenos { color: inherit; background-color: transparent; padding-left: 5px; padding-right: 5px; }
td.linenos .special { color: #000000; background-color: #ffffc0; padding-left: 5px; padding-right: 5px; }
span.linenos.special { color: #000000; background-color: #ffffc0; padding-left: 5px; padding-right: 5px; }
.highlight .hll { background-color: var(--jp-cell-editor-active-background) }
.highlight { background: var(--jp-cell-editor-background); color: var(--jp-mirror-editor-variable-color) }
.highlight .c { color: var(--jp-mirror-editor-comment-color); font-style: italic } /* Comment */
.highlight .err { color: var(--jp-mirror-editor-error-color) } /* Error */
.highlight .k { color: var(--jp-mirror-editor-keyword-color); font-weight: bold } /* Keyword */
.highlight .o { color: var(--jp-mirror-editor-operator-color); font-weight: bold } /* Operator */
.highlight .p { color: var(--jp-mirror-editor-punctuation-color) } /* Punctuation */
.highlight .ch { color: var(--jp-mirror-editor-comment-color); font-style: italic } /* Comment.Hashbang */
.highlight .cm { color: var(--jp-mirror-editor-comment-color); font-style: italic } /* Comment.Multiline */
.highlight .cp { color: var(--jp-mirror-editor-comment-color); font-style: italic } /* Comment.Preproc */
.highlight .cpf { color: var(--jp-mirror-editor-comment-color); font-style: italic } /* Comment.PreprocFile */
.highlight .c1 { color: var(--jp-mirror-editor-comment-color); font-style: italic } /* Comment.Single */
.highlight .cs { color: var(--jp-mirror-editor-comment-color); font-style: italic } /* Comment.Special */
.highlight .kc { color: var(--jp-mirror-editor-keyword-color); font-weight: bold } /* Keyword.Constant */
.highlight .kd { color: var(--jp-mirror-editor-keyword-color); font-weight: bold } /* Keyword.Declaration */
.highlight .kn { color: var(--jp-mirror-editor-keyword-color); font-weight: bold } /* Keyword.Namespace */
.highlight .kp { color: var(--jp-mirror-editor-keyword-color); font-weight: bold } /* Keyword.Pseudo */
.highlight .kr { color: var(--jp-mirror-editor-keyword-color); font-weight: bold } /* Keyword.Reserved */
.highlight .kt { color: var(--jp-mirror-editor-keyword-color); font-weight: bold } /* Keyword.Type */
.highlight .m { color: var(--jp-mirror-editor-number-color) } /* Literal.Number */
.highlight .s { color: var(--jp-mirror-editor-string-color) } /* Literal.String */
.highlight .ow { color: var(--jp-mirror-editor-operator-color); font-weight: bold } /* Operator.Word */
.highlight .pm { color: var(--jp-mirror-editor-punctuation-color) } /* Punctuation.Marker */
.highlight .w { color: var(--jp-mirror-editor-variable-color) } /* Text.Whitespace */
.highlight .mb { color: var(--jp-mirror-editor-number-color) } /* Literal.Number.Bin */
.highlight .mf { color: var(--jp-mirror-editor-number-color) } /* Literal.Number.Float */
.highlight .mh { color: var(--jp-mirror-editor-number-color) } /* Literal.Number.Hex */
.highlight .mi { color: var(--jp-mirror-editor-number-color) } /* Literal.Number.Integer */
.highlight .mo { color: var(--jp-mirror-editor-number-color) } /* Literal.Number.Oct */
.highlight .sa { color: var(--jp-mirror-editor-string-color) } /* Literal.String.Affix */
.highlight .sb { color: var(--jp-mirror-editor-string-color) } /* Literal.String.Backtick */
.highlight .sc { color: var(--jp-mirror-editor-string-color) } /* Literal.String.Char */
.highlight .dl { color: var(--jp-mirror-editor-string-color) } /* Literal.String.Delimiter */
.highlight .sd { color: var(--jp-mirror-editor-string-color) } /* Literal.String.Doc */
.highlight .s2 { color: var(--jp-mirror-editor-string-color) } /* Literal.String.Double */
.highlight .se { color: var(--jp-mirror-editor-string-color) } /* Literal.String.Escape */
.highlight .sh { color: var(--jp-mirror-editor-string-color) } /* Literal.String.Heredoc */
.highlight .si { color: var(--jp-mirror-editor-string-color) } /* Literal.String.Interpol */
.highlight .sx { color: var(--jp-mirror-editor-string-color) } /* Literal.String.Other */
.highlight .sr { color: var(--jp-mirror-editor-string-color) } /* Literal.String.Regex */
.highlight .s1 { color: var(--jp-mirror-editor-string-color) } /* Literal.String.Single */
.highlight .ss { color: var(--jp-mirror-editor-string-color) } /* Literal.String.Symbol */
.highlight .il { color: var(--jp-mirror-editor-number-color) } /* Literal.Number.Integer.Long */
  </style>
<style type="text/css">
/*-----------------------------------------------------------------------------
| Copyright (c) Jupyter Development Team.
| Distributed under the terms of the Modified BSD License.
|----------------------------------------------------------------------------*/

/*
 * Mozilla scrollbar styling
 */

/* use standard opaque scrollbars for most nodes */
[data-jp-theme-scrollbars='true'] {
  scrollbar-color: rgb(var(--jp-scrollbar-thumb-color))
    var(--jp-scrollbar-background-color);
}

/* for code nodes, use a transparent style of scrollbar. These selectors
 * will match lower in the tree, and so will override the above */
[data-jp-theme-scrollbars='true'] .CodeMirror-hscrollbar,
[data-jp-theme-scrollbars='true'] .CodeMirror-vscrollbar {
  scrollbar-color: rgba(var(--jp-scrollbar-thumb-color), 0.5) transparent;
}

/* tiny scrollbar */

.jp-scrollbar-tiny {
  scrollbar-color: rgba(var(--jp-scrollbar-thumb-color), 0.5) transparent;
  scrollbar-width: thin;
}

/* tiny scrollbar */

.jp-scrollbar-tiny::-webkit-scrollbar,
.jp-scrollbar-tiny::-webkit-scrollbar-corner {
  background-color: transparent;
  height: 4px;
  width: 4px;
}

.jp-scrollbar-tiny::-webkit-scrollbar-thumb {
  background: rgba(var(--jp-scrollbar-thumb-color), 0.5);
}

.jp-scrollbar-tiny::-webkit-scrollbar-track:horizontal {
  border-left: 0 solid transparent;
  border-right: 0 solid transparent;
}

.jp-scrollbar-tiny::-webkit-scrollbar-track:vertical {
  border-top: 0 solid transparent;
  border-bottom: 0 solid transparent;
}

/*
 * Lumino
 */

.lm-ScrollBar[data-orientation='horizontal'] {
  min-height: 16px;
  max-height: 16px;
  min-width: 45px;
  border-top: 1px solid #a0a0a0;
}

.lm-ScrollBar[data-orientation='vertical'] {
  min-width: 16px;
  max-width: 16px;
  min-height: 45px;
  border-left: 1px solid #a0a0a0;
}

.lm-ScrollBar-button {
  background-color: #f0f0f0;
  background-position: center center;
  min-height: 15px;
  max-height: 15px;
  min-width: 15px;
  max-width: 15px;
}

.lm-ScrollBar-button:hover {
  background-color: #dadada;
}

.lm-ScrollBar-button.lm-mod-active {
  background-color: #cdcdcd;
}

.lm-ScrollBar-track {
  background: #f0f0f0;
}

.lm-ScrollBar-thumb {
  background: #cdcdcd;
}

.lm-ScrollBar-thumb:hover {
  background: #bababa;
}

.lm-ScrollBar-thumb.lm-mod-active {
  background: #a0a0a0;
}

.lm-ScrollBar[data-orientation='horizontal'] .lm-ScrollBar-thumb {
  height: 100%;
  min-width: 15px;
  border-left: 1px solid #a0a0a0;
  border-right: 1px solid #a0a0a0;
}

.lm-ScrollBar[data-orientation='vertical'] .lm-ScrollBar-thumb {
  width: 100%;
  min-height: 15px;
  border-top: 1px solid #a0a0a0;
  border-bottom: 1px solid #a0a0a0;
}

.lm-ScrollBar[data-orientation='horizontal']
  .lm-ScrollBar-button[data-action='decrement'] {
  background-image: var(--jp-icon-caret-left);
  background-size: 17px;
}

.lm-ScrollBar[data-orientation='horizontal']
  .lm-ScrollBar-button[data-action='increment'] {
  background-image: var(--jp-icon-caret-right);
  background-size: 17px;
}

.lm-ScrollBar[data-orientation='vertical']
  .lm-ScrollBar-button[data-action='decrement'] {
  background-image: var(--jp-icon-caret-up);
  background-size: 17px;
}

.lm-ScrollBar[data-orientation='vertical']
  .lm-ScrollBar-button[data-action='increment'] {
  background-image: var(--jp-icon-caret-down);
  background-size: 17px;
}

/*
 * Copyright (c) Jupyter Development Team.
 * Distributed under the terms of the Modified BSD License.
 */

/*-----------------------------------------------------------------------------
| Copyright (c) Jupyter Development Team.
| Copyright (c) 2014-2017, PhosphorJS Contributors
|
| Distributed under the terms of the BSD 3-Clause License.
|
| The full license is in the file LICENSE, distributed with this software.
|----------------------------------------------------------------------------*/

.lm-Widget {
  box-sizing: border-box;
  position: relative;
  overflow: hidden;
}

.lm-Widget.lm-mod-hidden {
  display: none !important;
}

/*
 * Copyright (c) Jupyter Development Team.
 * Distributed under the terms of the Modified BSD License.
 */

.lm-AccordionPanel[data-orientation='horizontal'] > .lm-AccordionPanel-title {
  /* Title is rotated for horizontal accordion panel using CSS */
  display: block;
  transform-origin: top left;
  transform: rotate(-90deg) translate(-100%);
}

/*
 * Copyright (c) Jupyter Development Team.
 * Distributed under the terms of the Modified BSD License.
 */

/*-----------------------------------------------------------------------------
| Copyright (c) Jupyter Development Team.
| Copyright (c) 2014-2017, PhosphorJS Contributors
|
| Distributed under the terms of the BSD 3-Clause License.
|
| The full license is in the file LICENSE, distributed with this software.
|----------------------------------------------------------------------------*/

.lm-CommandPalette {
  display: flex;
  flex-direction: column;
  -webkit-user-select: none;
  -moz-user-select: none;
  -ms-user-select: none;
  user-select: none;
}

.lm-CommandPalette-search {
  flex: 0 0 auto;
}

.lm-CommandPalette-content {
  flex: 1 1 auto;
  margin: 0;
  padding: 0;
  min-height: 0;
  overflow: auto;
  list-style-type: none;
}

.lm-CommandPalette-header {
  overflow: hidden;
  white-space: nowrap;
  text-overflow: ellipsis;
}

.lm-CommandPalette-item {
  display: flex;
  flex-direction: row;
}

.lm-CommandPalette-itemIcon {
  flex: 0 0 auto;
}

.lm-CommandPalette-itemContent {
  flex: 1 1 auto;
  overflow: hidden;
}

.lm-CommandPalette-itemShortcut {
  flex: 0 0 auto;
}

.lm-CommandPalette-itemLabel {
  overflow: hidden;
  white-space: nowrap;
  text-overflow: ellipsis;
}

.lm-close-icon {
  border: 1px solid transparent;
  background-color: transparent;
  position: absolute;
  z-index: 1;
  right: 3%;
  top: 0;
  bottom: 0;
  margin: auto;
  padding: 7px 0;
  display: none;
  vertical-align: middle;
  outline: 0;
  cursor: pointer;
}
.lm-close-icon:after {
  content: 'X';
  display: block;
  width: 15px;
  height: 15px;
  text-align: center;
  color: #000;
  font-weight: normal;
  font-size: 12px;
  cursor: pointer;
}

/*
 * Copyright (c) Jupyter Development Team.
 * Distributed under the terms of the Modified BSD License.
 */

/*-----------------------------------------------------------------------------
| Copyright (c) Jupyter Development Team.
| Copyright (c) 2014-2017, PhosphorJS Contributors
|
| Distributed under the terms of the BSD 3-Clause License.
|
| The full license is in the file LICENSE, distributed with this software.
|----------------------------------------------------------------------------*/

.lm-DockPanel {
  z-index: 0;
}

.lm-DockPanel-widget {
  z-index: 0;
}

.lm-DockPanel-tabBar {
  z-index: 1;
}

.lm-DockPanel-handle {
  z-index: 2;
}

.lm-DockPanel-handle.lm-mod-hidden {
  display: none !important;
}

.lm-DockPanel-handle:after {
  position: absolute;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  content: '';
}

.lm-DockPanel-handle[data-orientation='horizontal'] {
  cursor: ew-resize;
}

.lm-DockPanel-handle[data-orientation='vertical'] {
  cursor: ns-resize;
}

.lm-DockPanel-handle[data-orientation='horizontal']:after {
  left: 50%;
  min-width: 8px;
  transform: translateX(-50%);
}

.lm-DockPanel-handle[data-orientation='vertical']:after {
  top: 50%;
  min-height: 8px;
  transform: translateY(-50%);
}

.lm-DockPanel-overlay {
  z-index: 3;
  box-sizing: border-box;
  pointer-events: none;
}

.lm-DockPanel-overlay.lm-mod-hidden {
  display: none !important;
}

/*
 * Copyright (c) Jupyter Development Team.
 * Distributed under the terms of the Modified BSD License.
 */

/*-----------------------------------------------------------------------------
| Copyright (c) Jupyter Development Team.
| Copyright (c) 2014-2017, PhosphorJS Contributors
|
| Distributed under the terms of the BSD 3-Clause License.
|
| The full license is in the file LICENSE, distributed with this software.
|----------------------------------------------------------------------------*/

.lm-Menu {
  z-index: 10000;
  position: absolute;
  white-space: nowrap;
  overflow-x: hidden;
  overflow-y: auto;
  outline: none;
  -webkit-user-select: none;
  -moz-user-select: none;
  -ms-user-select: none;
  user-select: none;
}

.lm-Menu-content {
  margin: 0;
  padding: 0;
  display: table;
  list-style-type: none;
}

.lm-Menu-item {
  display: table-row;
}

.lm-Menu-item.lm-mod-hidden,
.lm-Menu-item.lm-mod-collapsed {
  display: none !important;
}

.lm-Menu-itemIcon,
.lm-Menu-itemSubmenuIcon {
  display: table-cell;
  text-align: center;
}

.lm-Menu-itemLabel {
  display: table-cell;
  text-align: left;
}

.lm-Menu-itemShortcut {
  display: table-cell;
  text-align: right;
}

/*
 * Copyright (c) Jupyter Development Team.
 * Distributed under the terms of the Modified BSD License.
 */

/*-----------------------------------------------------------------------------
| Copyright (c) Jupyter Development Team.
| Copyright (c) 2014-2017, PhosphorJS Contributors
|
| Distributed under the terms of the BSD 3-Clause License.
|
| The full license is in the file LICENSE, distributed with this software.
|----------------------------------------------------------------------------*/

.lm-MenuBar {
  outline: none;
  -webkit-user-select: none;
  -moz-user-select: none;
  -ms-user-select: none;
  user-select: none;
}

.lm-MenuBar-content {
  margin: 0;
  padding: 0;
  display: flex;
  flex-direction: row;
  list-style-type: none;
}

.lm-MenuBar-item {
  box-sizing: border-box;
}

.lm-MenuBar-itemIcon,
.lm-MenuBar-itemLabel {
  display: inline-block;
}

/*
 * Copyright (c) Jupyter Development Team.
 * Distributed under the terms of the Modified BSD License.
 */

/*-----------------------------------------------------------------------------
| Copyright (c) Jupyter Development Team.
| Copyright (c) 2014-2017, PhosphorJS Contributors
|
| Distributed under the terms of the BSD 3-Clause License.
|
| The full license is in the file LICENSE, distributed with this software.
|----------------------------------------------------------------------------*/

.lm-ScrollBar {
  display: flex;
  -webkit-user-select: none;
  -moz-user-select: none;
  -ms-user-select: none;
  user-select: none;
}

.lm-ScrollBar[data-orientation='horizontal'] {
  flex-direction: row;
}

.lm-ScrollBar[data-orientation='vertical'] {
  flex-direction: column;
}

.lm-ScrollBar-button {
  box-sizing: border-box;
  flex: 0 0 auto;
}

.lm-ScrollBar-track {
  box-sizing: border-box;
  position: relative;
  overflow: hidden;
  flex: 1 1 auto;
}

.lm-ScrollBar-thumb {
  box-sizing: border-box;
  position: absolute;
}

/*
 * Copyright (c) Jupyter Development Team.
 * Distributed under the terms of the Modified BSD License.
 */

/*-----------------------------------------------------------------------------
| Copyright (c) Jupyter Development Team.
| Copyright (c) 2014-2017, PhosphorJS Contributors
|
| Distributed under the terms of the BSD 3-Clause License.
|
| The full license is in the file LICENSE, distributed with this software.
|----------------------------------------------------------------------------*/

.lm-SplitPanel-child {
  z-index: 0;
}

.lm-SplitPanel-handle {
  z-index: 1;
}

.lm-SplitPanel-handle.lm-mod-hidden {
  display: none !important;
}

.lm-SplitPanel-handle:after {
  position: absolute;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  content: '';
}

.lm-SplitPanel[data-orientation='horizontal'] > .lm-SplitPanel-handle {
  cursor: ew-resize;
}

.lm-SplitPanel[data-orientation='vertical'] > .lm-SplitPanel-handle {
  cursor: ns-resize;
}

.lm-SplitPanel[data-orientation='horizontal'] > .lm-SplitPanel-handle:after {
  left: 50%;
  min-width: 8px;
  transform: translateX(-50%);
}

.lm-SplitPanel[data-orientation='vertical'] > .lm-SplitPanel-handle:after {
  top: 50%;
  min-height: 8px;
  transform: translateY(-50%);
}

/*
 * Copyright (c) Jupyter Development Team.
 * Distributed under the terms of the Modified BSD License.
 */

/*-----------------------------------------------------------------------------
| Copyright (c) Jupyter Development Team.
| Copyright (c) 2014-2017, PhosphorJS Contributors
|
| Distributed under the terms of the BSD 3-Clause License.
|
| The full license is in the file LICENSE, distributed with this software.
|----------------------------------------------------------------------------*/

.lm-TabBar {
  display: flex;
  -webkit-user-select: none;
  -moz-user-select: none;
  -ms-user-select: none;
  user-select: none;
}

.lm-TabBar[data-orientation='horizontal'] {
  flex-direction: row;
  align-items: flex-end;
}

.lm-TabBar[data-orientation='vertical'] {
  flex-direction: column;
  align-items: flex-end;
}

.lm-TabBar-content {
  margin: 0;
  padding: 0;
  display: flex;
  flex: 1 1 auto;
  list-style-type: none;
}

.lm-TabBar[data-orientation='horizontal'] > .lm-TabBar-content {
  flex-direction: row;
}

.lm-TabBar[data-orientation='vertical'] > .lm-TabBar-content {
  flex-direction: column;
}

.lm-TabBar-tab {
  display: flex;
  flex-direction: row;
  box-sizing: border-box;
  overflow: hidden;
  touch-action: none; /* Disable native Drag/Drop */
}

.lm-TabBar-tabIcon,
.lm-TabBar-tabCloseIcon {
  flex: 0 0 auto;
}

.lm-TabBar-tabLabel {
  flex: 1 1 auto;
  overflow: hidden;
  white-space: nowrap;
}

.lm-TabBar-tabInput {
  user-select: all;
  width: 100%;
  box-sizing: border-box;
}

.lm-TabBar-tab.lm-mod-hidden {
  display: none !important;
}

.lm-TabBar-addButton.lm-mod-hidden {
  display: none !important;
}

.lm-TabBar.lm-mod-dragging .lm-TabBar-tab {
  position: relative;
}

.lm-TabBar.lm-mod-dragging[data-orientation='horizontal'] .lm-TabBar-tab {
  left: 0;
  transition: left 150ms ease;
}

.lm-TabBar.lm-mod-dragging[data-orientation='vertical'] .lm-TabBar-tab {
  top: 0;
  transition: top 150ms ease;
}

.lm-TabBar.lm-mod-dragging .lm-TabBar-tab.lm-mod-dragging {
  transition: none;
}

.lm-TabBar-tabLabel .lm-TabBar-tabInput {
  user-select: all;
  width: 100%;
  box-sizing: border-box;
  background: inherit;
}

/*
 * Copyright (c) Jupyter Development Team.
 * Distributed under the terms of the Modified BSD License.
 */

/*-----------------------------------------------------------------------------
| Copyright (c) Jupyter Development Team.
| Copyright (c) 2014-2017, PhosphorJS Contributors
|
| Distributed under the terms of the BSD 3-Clause License.
|
| The full license is in the file LICENSE, distributed with this software.
|----------------------------------------------------------------------------*/

.lm-TabPanel-tabBar {
  z-index: 1;
}

.lm-TabPanel-stackedPanel {
  z-index: 0;
}

/*
 * Copyright (c) Jupyter Development Team.
 * Distributed under the terms of the Modified BSD License.
 */

/*-----------------------------------------------------------------------------
| Copyright (c) Jupyter Development Team.
| Copyright (c) 2014-2017, PhosphorJS Contributors
|
| Distributed under the terms of the BSD 3-Clause License.
|
| The full license is in the file LICENSE, distributed with this software.
|----------------------------------------------------------------------------*/

/*-----------------------------------------------------------------------------
| Copyright (c) Jupyter Development Team.
| Distributed under the terms of the Modified BSD License.
|----------------------------------------------------------------------------*/

.jp-Collapse {
  display: flex;
  flex-direction: column;
  align-items: stretch;
}

.jp-Collapse-header {
  padding: 1px 12px;
  background-color: var(--jp-layout-color1);
  border-bottom: solid var(--jp-border-width) var(--jp-border-color2);
  color: var(--jp-ui-font-color1);
  cursor: pointer;
  display: flex;
  align-items: center;
  font-size: var(--jp-ui-font-size0);
  font-weight: 600;
  text-transform: uppercase;
  user-select: none;
}

.jp-Collapser-icon {
  height: 16px;
}

.jp-Collapse-header-collapsed .jp-Collapser-icon {
  transform: rotate(-90deg);
  margin: auto 0;
}

.jp-Collapser-title {
  line-height: 25px;
}

.jp-Collapse-contents {
  padding: 0 12px;
  background-color: var(--jp-layout-color1);
  color: var(--jp-ui-font-color1);
  overflow: auto;
}

/*-----------------------------------------------------------------------------
| Copyright (c) Jupyter Development Team.
| Distributed under the terms of the Modified BSD License.
|----------------------------------------------------------------------------*/

/* This file was auto-generated by ensureUiComponents() in @jupyterlab/buildutils */

/**
 * (DEPRECATED) Support for consuming icons as CSS background images
 */

/* Icons urls */

:root {
  --jp-icon-add-above: url(data:image/svg+xml;base64,PHN2ZyB3aWR0aD0iMTQiIGhlaWdodD0iMTQiIHZpZXdCb3g9IjAgMCAxNCAxNCIgZmlsbD0ibm9uZSIgeG1sbnM9Imh0dHA6Ly93d3cudzMub3JnLzIwMDAvc3ZnIj4KPGcgY2xpcC1wYXRoPSJ1cmwoI2NsaXAwXzEzN18xOTQ5MikiPgo8cGF0aCBjbGFzcz0ianAtaWNvbjMiIGQ9Ik00Ljc1IDQuOTMwNjZINi42MjVWNi44MDU2NkM2LjYyNSA3LjAxMTkxIDYuNzkzNzUgNy4xODA2NiA3IDcuMTgwNjZDNy4yMDYyNSA3LjE4MDY2IDcuMzc1IDcuMDExOTEgNy4zNzUgNi44MDU2NlY0LjkzMDY2SDkuMjVDOS40NTYyNSA0LjkzMDY2IDkuNjI1IDQuNzYxOTEgOS42MjUgNC41NTU2NkM5LjYyNSA0LjM0OTQxIDkuNDU2MjUgNC4xODA2NiA5LjI1IDQuMTgwNjZINy4zNzVWMi4zMDU2NkM3LjM3NSAyLjA5OTQxIDcuMjA2MjUgMS45MzA2NiA3IDEuOTMwNjZDNi43OTM3NSAxLjkzMDY2IDYuNjI1IDIuMDk5NDEgNi42MjUgMi4zMDU2NlY0LjE4MDY2SDQuNzVDNC41NDM3NSA0LjE4MDY2IDQuMzc1IDQuMzQ5NDEgNC4zNzUgNC41NTU2NkM0LjM3NSA0Ljc2MTkxIDQuNTQzNzUgNC45MzA2NiA0Ljc1IDQuOTMwNjZaIiBmaWxsPSIjNjE2MTYxIiBzdHJva2U9IiM2MTYxNjEiIHN0cm9rZS13aWR0aD0iMC43Ii8+CjwvZz4KPHBhdGggY2xhc3M9ImpwLWljb24zIiBmaWxsLXJ1bGU9ImV2ZW5vZGQiIGNsaXAtcnVsZT0iZXZlbm9kZCIgZD0iTTExLjUgOS41VjExLjVMMi41IDExLjVWOS41TDExLjUgOS41Wk0xMiA4QzEyLjU1MjMgOCAxMyA4LjQ0NzcyIDEzIDlWMTJDMTMgMTIuNTUyMyAxMi41NTIzIDEzIDEyIDEzTDIgMTNDMS40NDc3MiAxMyAxIDEyLjU1MjMgMSAxMlY5QzEgOC40NDc3MiAxLjQ0NzcxIDggMiA4TDEyIDhaIiBmaWxsPSIjNjE2MTYxIi8+CjxkZWZzPgo8Y2xpcFBhdGggaWQ9ImNsaXAwXzEzN18xOTQ5MiI+CjxyZWN0IGNsYXNzPSJqcC1pY29uMyIgd2lkdGg9IjYiIGhlaWdodD0iNiIgZmlsbD0id2hpdGUiIHRyYW5zZm9ybT0ibWF0cml4KC0xIDAgMCAxIDEwIDEuNTU1NjYpIi8+CjwvY2xpcFBhdGg+CjwvZGVmcz4KPC9zdmc+Cg==);
  --jp-icon-add-below: url(data:image/svg+xml;base64,PHN2ZyB3aWR0aD0iMTQiIGhlaWdodD0iMTQiIHZpZXdCb3g9IjAgMCAxNCAxNCIgZmlsbD0ibm9uZSIgeG1sbnM9Imh0dHA6Ly93d3cudzMub3JnLzIwMDAvc3ZnIj4KPGcgY2xpcC1wYXRoPSJ1cmwoI2NsaXAwXzEzN18xOTQ5OCkiPgo8cGF0aCBjbGFzcz0ianAtaWNvbjMiIGQ9Ik05LjI1IDEwLjA2OTNMNy4zNzUgMTAuMDY5M0w3LjM3NSA4LjE5NDM0QzcuMzc1IDcuOTg4MDkgNy4yMDYyNSA3LjgxOTM0IDcgNy44MTkzNEM2Ljc5Mzc1IDcuODE5MzQgNi42MjUgNy45ODgwOSA2LjYyNSA4LjE5NDM0TDYuNjI1IDEwLjA2OTNMNC43NSAxMC4wNjkzQzQuNTQzNzUgMTAuMDY5MyA0LjM3NSAxMC4yMzgxIDQuMzc1IDEwLjQ0NDNDNC4zNzUgMTAuNjUwNiA0LjU0Mzc1IDEwLjgxOTMgNC43NSAxMC44MTkzTDYuNjI1IDEwLjgxOTNMNi42MjUgMTIuNjk0M0M2LjYyNSAxMi45MDA2IDYuNzkzNzUgMTMuMDY5MyA3IDEzLjA2OTNDNy4yMDYyNSAxMy4wNjkzIDcuMzc1IDEyLjkwMDYgNy4zNzUgMTIuNjk0M0w3LjM3NSAxMC44MTkzTDkuMjUgMTAuODE5M0M5LjQ1NjI1IDEwLjgxOTMgOS42MjUgMTAuNjUwNiA5LjYyNSAxMC40NDQzQzkuNjI1IDEwLjIzODEgOS40NTYyNSAxMC4wNjkzIDkuMjUgMTAuMDY5M1oiIGZpbGw9IiM2MTYxNjEiIHN0cm9rZT0iIzYxNjE2MSIgc3Ryb2tlLXdpZHRoPSIwLjciLz4KPC9nPgo8cGF0aCBjbGFzcz0ianAtaWNvbjMiIGZpbGwtcnVsZT0iZXZlbm9kZCIgY2xpcC1ydWxlPSJldmVub2RkIiBkPSJNMi41IDUuNUwyLjUgMy41TDExLjUgMy41TDExLjUgNS41TDIuNSA1LjVaTTIgN0MxLjQ0NzcyIDcgMSA2LjU1MjI4IDEgNkwxIDNDMSAyLjQ0NzcyIDEuNDQ3NzIgMiAyIDJMMTIgMkMxMi41NTIzIDIgMTMgMi40NDc3MiAxMyAzTDEzIDZDMTMgNi41NTIyOSAxMi41NTIzIDcgMTIgN0wyIDdaIiBmaWxsPSIjNjE2MTYxIi8+CjxkZWZzPgo8Y2xpcFBhdGggaWQ9ImNsaXAwXzEzN18xOTQ5OCI+CjxyZWN0IGNsYXNzPSJqcC1pY29uMyIgd2lkdGg9IjYiIGhlaWdodD0iNiIgZmlsbD0id2hpdGUiIHRyYW5zZm9ybT0ibWF0cml4KDEgMS43NDg0NmUtMDcgMS43NDg0NmUtMDcgLTEgNCAxMy40NDQzKSIvPgo8L2NsaXBQYXRoPgo8L2RlZnM+Cjwvc3ZnPgo=);
  --jp-icon-add: url(data:image/svg+xml;base64,PHN2ZyB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIHdpZHRoPSIxNiIgdmlld0JveD0iMCAwIDI0IDI0Ij4KICA8ZyBjbGFzcz0ianAtaWNvbjMiIGZpbGw9IiM2MTYxNjEiPgogICAgPHBhdGggZD0iTTE5IDEzaC02djZoLTJ2LTZINXYtMmg2VjVoMnY2aDZ2MnoiLz4KICA8L2c+Cjwvc3ZnPgo=);
  --jp-icon-bell: url(data:image/svg+xml;base64,PHN2ZyB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIHdpZHRoPSIxNiIgdmlld0JveD0iMCAwIDE2IDE2IiB2ZXJzaW9uPSIxLjEiPgogICA8cGF0aCBjbGFzcz0ianAtaWNvbjIganAtaWNvbi1zZWxlY3RhYmxlIiBmaWxsPSIjMzMzMzMzIgogICAgICBkPSJtOCAwLjI5Yy0xLjQgMC0yLjcgMC43My0zLjYgMS44LTEuMiAxLjUtMS40IDMuNC0xLjUgNS4yLTAuMTggMi4yLTAuNDQgNC0yLjMgNS4zbDAuMjggMS4zaDVjMC4wMjYgMC42NiAwLjMyIDEuMSAwLjcxIDEuNSAwLjg0IDAuNjEgMiAwLjYxIDIuOCAwIDAuNTItMC40IDAuNi0xIDAuNzEtMS41aDVsMC4yOC0xLjNjLTEuOS0wLjk3LTIuMi0zLjMtMi4zLTUuMy0wLjEzLTEuOC0wLjI2LTMuNy0xLjUtNS4yLTAuODUtMS0yLjItMS44LTMuNi0xLjh6bTAgMS40YzAuODggMCAxLjkgMC41NSAyLjUgMS4zIDAuODggMS4xIDEuMSAyLjcgMS4yIDQuNCAwLjEzIDEuNyAwLjIzIDMuNiAxLjMgNS4yaC0xMGMxLjEtMS42IDEuMi0zLjQgMS4zLTUuMiAwLjEzLTEuNyAwLjMtMy4zIDEuMi00LjQgMC41OS0wLjcyIDEuNi0xLjMgMi41LTEuM3ptLTAuNzQgMTJoMS41Yy0wLjAwMTUgMC4yOCAwLjAxNSAwLjc5LTAuNzQgMC43OS0wLjczIDAuMDAxNi0wLjcyLTAuNTMtMC43NC0wLjc5eiIgLz4KPC9zdmc+Cg==);
  --jp-icon-bug-dot: url(data:image/svg+xml;base64,PHN2ZyB3aWR0aD0iMjQiIGhlaWdodD0iMjQiIHZpZXdCb3g9IjAgMCAyNCAyNCIgZmlsbD0ibm9uZSIgeG1sbnM9Imh0dHA6Ly93d3cudzMub3JnLzIwMDAvc3ZnIj4KICAgIDxnIGNsYXNzPSJqcC1pY29uMyBqcC1pY29uLXNlbGVjdGFibGUiIGZpbGw9IiM2MTYxNjEiPgogICAgICAgIDxwYXRoIGZpbGwtcnVsZT0iZXZlbm9kZCIgY2xpcC1ydWxlPSJldmVub2RkIiBkPSJNMTcuMTkgOEgyMFYxMEgxNy45MUMxNy45NiAxMC4zMyAxOCAxMC42NiAxOCAxMVYxMkgyMFYxNEgxOC41SDE4VjE0LjAyNzVDMTUuNzUgMTQuMjc2MiAxNCAxNi4xODM3IDE0IDE4LjVDMTQgMTkuMjA4IDE0LjE2MzUgMTkuODc3OSAxNC40NTQ5IDIwLjQ3MzlDMTMuNzA2MyAyMC44MTE3IDEyLjg3NTcgMjEgMTIgMjFDOS43OCAyMSA3Ljg1IDE5Ljc5IDYuODEgMThINFYxNkg2LjA5QzYuMDQgMTUuNjcgNiAxNS4zNCA2IDE1VjE0SDRWMTJINlYxMUM2IDEwLjY2IDYuMDQgMTAuMzMgNi4wOSAxMEg0VjhINi44MUM3LjI2IDcuMjIgNy44OCA2LjU1IDguNjIgNi4wNEw3IDQuNDFMOC40MSAzTDEwLjU5IDUuMTdDMTEuMDQgNS4wNiAxMS41MSA1IDEyIDVDMTIuNDkgNSAxMi45NiA1LjA2IDEzLjQyIDUuMTdMMTUuNTkgM0wxNyA0LjQxTDE1LjM3IDYuMDRDMTYuMTIgNi41NSAxNi43NCA3LjIyIDE3LjE5IDhaTTEwIDE2SDE0VjE0SDEwVjE2Wk0xMCAxMkgxNFYxMEgxMFYxMloiIGZpbGw9IiM2MTYxNjEiLz4KICAgICAgICA8cGF0aCBkPSJNMjIgMTguNUMyMiAyMC40MzMgMjAuNDMzIDIyIDE4LjUgMjJDMTYuNTY3IDIyIDE1IDIwLjQzMyAxNSAxOC41QzE1IDE2LjU2NyAxNi41NjcgMTUgMTguNSAxNUMyMC40MzMgMTUgMjIgMTYuNTY3IDIyIDE4LjVaIiBmaWxsPSIjNjE2MTYxIi8+CiAgICA8L2c+Cjwvc3ZnPgo=);
  --jp-icon-bug: url(data:image/svg+xml;base64,PHN2ZyB2aWV3Qm94PSIwIDAgMjQgMjQiIHdpZHRoPSIxNiIgeG1sbnM9Imh0dHA6Ly93d3cudzMub3JnLzIwMDAvc3ZnIj4KICA8ZyBjbGFzcz0ianAtaWNvbjMganAtaWNvbi1zZWxlY3RhYmxlIiBmaWxsPSIjNjE2MTYxIj4KICAgIDxwYXRoIGQ9Ik0yMCA4aC0yLjgxYy0uNDUtLjc4LTEuMDctMS40NS0xLjgyLTEuOTZMMTcgNC40MSAxNS41OSAzbC0yLjE3IDIuMTdDMTIuOTYgNS4wNiAxMi40OSA1IDEyIDVjLS40OSAwLS45Ni4wNi0xLjQxLjE3TDguNDEgMyA3IDQuNDFsMS42MiAxLjYzQzcuODggNi41NSA3LjI2IDcuMjIgNi44MSA4SDR2MmgyLjA5Yy0uMDUuMzMtLjA5LjY2LS4wOSAxdjFINHYyaDJ2MWMwIC4zNC4wNC42Ny4wOSAxSDR2MmgyLjgxYzEuMDQgMS43OSAyLjk3IDMgNS4xOSAzczQuMTUtMS4yMSA1LjE5LTNIMjB2LTJoLTIuMDljLjA1LS4zMy4wOS0uNjYuMDktMXYtMWgydi0yaC0ydi0xYzAtLjM0LS4wNC0uNjctLjA5LTFIMjBWOHptLTYgOGgtNHYtMmg0djJ6bTAtNGgtNHYtMmg0djJ6Ii8+CiAgPC9nPgo8L3N2Zz4K);
  --jp-icon-build: url(data:image/svg+xml;base64,PHN2ZyB3aWR0aD0iMTYiIHZpZXdCb3g9IjAgMCAyNCAyNCIgeG1sbnM9Imh0dHA6Ly93d3cudzMub3JnLzIwMDAvc3ZnIj4KICA8ZyBjbGFzcz0ianAtaWNvbjMiIGZpbGw9IiM2MTYxNjEiPgogICAgPHBhdGggZD0iTTE0LjkgMTcuNDVDMTYuMjUgMTcuNDUgMTcuMzUgMTYuMzUgMTcuMzUgMTVDMTcuMzUgMTMuNjUgMTYuMjUgMTIuNTUgMTQuOSAxMi41NUMxMy41NCAxMi41NSAxMi40NSAxMy42NSAxMi40NSAxNUMxMi40NSAxNi4zNSAxMy41NCAxNy40NSAxNC45IDE3LjQ1Wk0yMC4xIDE1LjY4TDIxLjU4IDE2Ljg0QzIxLjcxIDE2Ljk1IDIxLjc1IDE3LjEzIDIxLjY2IDE3LjI5TDIwLjI2IDE5LjcxQzIwLjE3IDE5Ljg2IDIwIDE5LjkyIDE5LjgzIDE5Ljg2TDE4LjA5IDE5LjE2QzE3LjczIDE5LjQ0IDE3LjMzIDE5LjY3IDE2LjkxIDE5Ljg1TDE2LjY0IDIxLjdDMTYuNjIgMjEuODcgMTYuNDcgMjIgMTYuMyAyMkgxMy41QzEzLjMyIDIyIDEzLjE4IDIxLjg3IDEzLjE1IDIxLjdMMTIuODkgMTkuODVDMTIuNDYgMTkuNjcgMTIuMDcgMTkuNDQgMTEuNzEgMTkuMTZMOS45NjAwMiAxOS44NkM5LjgxMDAyIDE5LjkyIDkuNjIwMDIgMTkuODYgOS41NDAwMiAxOS43MUw4LjE0MDAyIDE3LjI5QzguMDUwMDIgMTcuMTMgOC4wOTAwMiAxNi45NSA4LjIyMDAyIDE2Ljg0TDkuNzAwMDIgMTUuNjhMOS42NTAwMSAxNUw5LjcwMDAyIDE0LjMxTDguMjIwMDIgMTMuMTZDOC4wOTAwMiAxMy4wNSA4LjA1MDAyIDEyLjg2IDguMTQwMDIgMTIuNzFMOS41NDAwMiAxMC4yOUM5LjYyMDAyIDEwLjEzIDkuODEwMDIgMTAuMDcgOS45NjAwMiAxMC4xM0wxMS43MSAxMC44NEMxMi4wNyAxMC41NiAxMi40NiAxMC4zMiAxMi44OSAxMC4xNUwxMy4xNSA4LjI4OTk4QzEzLjE4IDguMTI5OTggMTMuMzIgNy45OTk5OCAxMy41IDcuOTk5OThIMTYuM0MxNi40NyA3Ljk5OTk4IDE2LjYyIDguMTI5OTggMTYuNjQgOC4yODk5OEwxNi45MSAxMC4xNUMxNy4zMyAxMC4zMiAxNy43MyAxMC41NiAxOC4wOSAxMC44NEwxOS44MyAxMC4xM0MyMCAxMC4wNyAyMC4xNyAxMC4xMyAyMC4yNiAxMC4yOUwyMS42NiAxMi43MUMyMS43NSAxMi44NiAyMS43MSAxMy4wNSAyMS41OCAxMy4xNkwyMC4xIDE0LjMxTDIwLjE1IDE1TDIwLjEgMTUuNjhaIi8+CiAgICA8cGF0aCBkPSJNNy4zMjk2NiA3LjQ0NDU0QzguMDgzMSA3LjAwOTU0IDguMzM5MzIgNi4wNTMzMiA3LjkwNDMyIDUuMjk5ODhDNy40NjkzMiA0LjU0NjQzIDYuNTA4MSA0LjI4MTU2IDUuNzU0NjYgNC43MTY1NkM1LjM5MTc2IDQuOTI2MDggNS4xMjY5NSA1LjI3MTE4IDUuMDE4NDkgNS42NzU5NEM0LjkxMDA0IDYuMDgwNzEgNC45NjY4MiA2LjUxMTk4IDUuMTc2MzQgNi44NzQ4OEM1LjYxMTM0IDcuNjI4MzIgNi41NzYyMiA3Ljg3OTU0IDcuMzI5NjYgNy40NDQ1NFpNOS42NTcxOCA0Ljc5NTkzTDEwLjg2NzIgNC45NTE3OUMxMC45NjI4IDQuOTc3NDEgMTEuMDQwMiA1LjA3MTMzIDExLjAzODIgNS4xODc5M0wxMS4wMzg4IDYuOTg4OTNDMTEuMDQ1NSA3LjEwMDU0IDEwLjk2MTYgNy4xOTUxOCAxMC44NTUgNy4yMTA1NEw5LjY2MDAxIDcuMzgwODNMOS4yMzkxNSA4LjEzMTg4TDkuNjY5NjEgOS4yNTc0NUM5LjcwNzI5IDkuMzYyNzEgOS42NjkzNCA5LjQ3Njk5IDkuNTc0MDggOS41MzE5OUw4LjAxNTIzIDEwLjQzMkM3LjkxMTMxIDEwLjQ5MiA3Ljc5MzM3IDEwLjQ2NzcgNy43MjEwNSAxMC4zODI0TDYuOTg3NDggOS40MzE4OEw2LjEwOTMxIDkuNDMwODNMNS4zNDcwNCAxMC4zOTA1QzUuMjg5MDkgMTAuNDcwMiA1LjE3MzgzIDEwLjQ5MDUgNS4wNzE4NyAxMC40MzM5TDMuNTEyNDUgOS41MzI5M0MzLjQxMDQ5IDkuNDc2MzMgMy4zNzY0NyA5LjM1NzQxIDMuNDEwNzUgOS4yNTY3OUwzLjg2MzQ3IDguMTQwOTNMMy42MTc0OSA3Ljc3NDg4TDMuNDIzNDcgNy4zNzg4M0wyLjIzMDc1IDcuMjEyOTdDMi4xMjY0NyA3LjE5MjM1IDIuMDQwNDkgNy4xMDM0MiAyLjA0MjQ1IDYuOTg2ODJMMi4wNDE4NyA1LjE4NTgyQzIuMDQzODMgNS4wNjkyMiAyLjExOTA5IDQuOTc5NTggMi4yMTcwNCA0Ljk2OTIyTDMuNDIwNjUgNC43OTM5M0wzLjg2NzQ5IDQuMDI3ODhMMy40MTEwNSAyLjkxNzMxQzMuMzczMzcgMi44MTIwNCAzLjQxMTMxIDIuNjk3NzYgMy41MTUyMyAyLjYzNzc2TDUuMDc0MDggMS43Mzc3NkM1LjE2OTM0IDEuNjgyNzYgNS4yODcyOSAxLjcwNzA0IDUuMzU5NjEgMS43OTIzMUw2LjExOTE1IDIuNzI3ODhMNi45ODAwMSAyLjczODkzTDcuNzI0OTYgMS43ODkyMkM3Ljc5MTU2IDEuNzA0NTggNy45MTU0OCAxLjY3OTIyIDguMDA4NzkgMS43NDA4Mkw5LjU2ODIxIDIuNjQxODJDOS42NzAxNyAyLjY5ODQyIDkuNzEyODUgMi44MTIzNCA5LjY4NzIzIDIuOTA3OTdMOS4yMTcxOCA0LjAzMzgzTDkuNDYzMTYgNC4zOTk4OEw5LjY1NzE4IDQuNzk1OTNaIi8+CiAgPC9nPgo8L3N2Zz4K);
  --jp-icon-caret-down-empty-thin: url(data:image/svg+xml;base64,PHN2ZyB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIHdpZHRoPSIxNiIgdmlld0JveD0iMCAwIDIwIDIwIj4KCTxnIGNsYXNzPSJqcC1pY29uMyIgZmlsbD0iIzYxNjE2MSIgc2hhcGUtcmVuZGVyaW5nPSJnZW9tZXRyaWNQcmVjaXNpb24iPgoJCTxwb2x5Z29uIGNsYXNzPSJzdDEiIHBvaW50cz0iOS45LDEzLjYgMy42LDcuNCA0LjQsNi42IDkuOSwxMi4yIDE1LjQsNi43IDE2LjEsNy40ICIvPgoJPC9nPgo8L3N2Zz4K);
  --jp-icon-caret-down-empty: url(data:image/svg+xml;base64,PHN2ZyB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIHdpZHRoPSIxNiIgdmlld0JveD0iMCAwIDE4IDE4Ij4KICA8ZyBjbGFzcz0ianAtaWNvbjMiIGZpbGw9IiM2MTYxNjEiIHNoYXBlLXJlbmRlcmluZz0iZ2VvbWV0cmljUHJlY2lzaW9uIj4KICAgIDxwYXRoIGQ9Ik01LjIsNS45TDksOS43bDMuOC0zLjhsMS4yLDEuMmwtNC45LDVsLTQuOS01TDUuMiw1Ljl6Ii8+CiAgPC9nPgo8L3N2Zz4K);
  --jp-icon-caret-down: url(data:image/svg+xml;base64,PHN2ZyB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIHdpZHRoPSIxNiIgdmlld0JveD0iMCAwIDE4IDE4Ij4KICA8ZyBjbGFzcz0ianAtaWNvbjMiIGZpbGw9IiM2MTYxNjEiIHNoYXBlLXJlbmRlcmluZz0iZ2VvbWV0cmljUHJlY2lzaW9uIj4KICAgIDxwYXRoIGQ9Ik01LjIsNy41TDksMTEuMmwzLjgtMy44SDUuMnoiLz4KICA8L2c+Cjwvc3ZnPgo=);
  --jp-icon-caret-left: url(data:image/svg+xml;base64,PHN2ZyB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIHdpZHRoPSIxNiIgdmlld0JveD0iMCAwIDE4IDE4Ij4KCTxnIGNsYXNzPSJqcC1pY29uMyIgZmlsbD0iIzYxNjE2MSIgc2hhcGUtcmVuZGVyaW5nPSJnZW9tZXRyaWNQcmVjaXNpb24iPgoJCTxwYXRoIGQ9Ik0xMC44LDEyLjhMNy4xLDlsMy44LTMuOGwwLDcuNkgxMC44eiIvPgogIDwvZz4KPC9zdmc+Cg==);
  --jp-icon-caret-right: url(data:image/svg+xml;base64,PHN2ZyB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIHdpZHRoPSIxNiIgdmlld0JveD0iMCAwIDE4IDE4Ij4KICA8ZyBjbGFzcz0ianAtaWNvbjMiIGZpbGw9IiM2MTYxNjEiIHNoYXBlLXJlbmRlcmluZz0iZ2VvbWV0cmljUHJlY2lzaW9uIj4KICAgIDxwYXRoIGQ9Ik03LjIsNS4yTDEwLjksOWwtMy44LDMuOFY1LjJINy4yeiIvPgogIDwvZz4KPC9zdmc+Cg==);
  --jp-icon-caret-up-empty-thin: url(data:image/svg+xml;base64,PHN2ZyB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIHdpZHRoPSIxNiIgdmlld0JveD0iMCAwIDIwIDIwIj4KCTxnIGNsYXNzPSJqcC1pY29uMyIgZmlsbD0iIzYxNjE2MSIgc2hhcGUtcmVuZGVyaW5nPSJnZW9tZXRyaWNQcmVjaXNpb24iPgoJCTxwb2x5Z29uIGNsYXNzPSJzdDEiIHBvaW50cz0iMTUuNCwxMy4zIDkuOSw3LjcgNC40LDEzLjIgMy42LDEyLjUgOS45LDYuMyAxNi4xLDEyLjYgIi8+Cgk8L2c+Cjwvc3ZnPgo=);
  --jp-icon-caret-up: url(data:image/svg+xml;base64,PHN2ZyB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIHdpZHRoPSIxNiIgdmlld0JveD0iMCAwIDE4IDE4Ij4KCTxnIGNsYXNzPSJqcC1pY29uMyIgZmlsbD0iIzYxNjE2MSIgc2hhcGUtcmVuZGVyaW5nPSJnZW9tZXRyaWNQcmVjaXNpb24iPgoJCTxwYXRoIGQ9Ik01LjIsMTAuNUw5LDYuOGwzLjgsMy44SDUuMnoiLz4KICA8L2c+Cjwvc3ZnPgo=);
  --jp-icon-case-sensitive: url(data:image/svg+xml;base64,PHN2ZyB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIHdpZHRoPSIxNiIgdmlld0JveD0iMCAwIDIwIDIwIj4KICA8ZyBjbGFzcz0ianAtaWNvbjIiIGZpbGw9IiM0MTQxNDEiPgogICAgPHJlY3QgeD0iMiIgeT0iMiIgd2lkdGg9IjE2IiBoZWlnaHQ9IjE2Ii8+CiAgPC9nPgogIDxnIGNsYXNzPSJqcC1pY29uLWFjY2VudDIiIGZpbGw9IiNGRkYiPgogICAgPHBhdGggZD0iTTcuNiw4aDAuOWwzLjUsOGgtMS4xTDEwLDE0SDZsLTAuOSwySDRMNy42LDh6IE04LDkuMUw2LjQsMTNoMy4yTDgsOS4xeiIvPgogICAgPHBhdGggZD0iTTE2LjYsOS44Yy0wLjIsMC4xLTAuNCwwLjEtMC43LDAuMWMtMC4yLDAtMC40LTAuMS0wLjYtMC4yYy0wLjEtMC4xLTAuMi0wLjQtMC4yLTAuNyBjLTAuMywwLjMtMC42LDAuNS0wLjksMC43Yy0wLjMsMC4xLTAuNywwLjItMS4xLDAuMmMtMC4zLDAtMC41LDAtMC43LTAuMWMtMC4yLTAuMS0wLjQtMC4yLTAuNi0wLjNjLTAuMi0wLjEtMC4zLTAuMy0wLjQtMC41IGMtMC4xLTAuMi0wLjEtMC40LTAuMS0wLjdjMC0wLjMsMC4xLTAuNiwwLjItMC44YzAuMS0wLjIsMC4zLTAuNCwwLjQtMC41QzEyLDcsMTIuMiw2LjksMTIuNSw2LjhjMC4yLTAuMSwwLjUtMC4xLDAuNy0wLjIgYzAuMy0wLjEsMC41LTAuMSwwLjctMC4xYzAuMiwwLDAuNC0wLjEsMC42LTAuMWMwLjIsMCwwLjMtMC4xLDAuNC0wLjJjMC4xLTAuMSwwLjItMC4yLDAuMi0wLjRjMC0xLTEuMS0xLTEuMy0xIGMtMC40LDAtMS40LDAtMS40LDEuMmgtMC45YzAtMC40LDAuMS0wLjcsMC4yLTFjMC4xLTAuMiwwLjMtMC40LDAuNS0wLjZjMC4yLTAuMiwwLjUtMC4zLDAuOC0wLjNDMTMuMyw0LDEzLjYsNCwxMy45LDQgYzAuMywwLDAuNSwwLDAuOCwwLjFjMC4zLDAsMC41LDAuMSwwLjcsMC4yYzAuMiwwLjEsMC40LDAuMywwLjUsMC41QzE2LDUsMTYsNS4yLDE2LDUuNnYyLjljMCwwLjIsMCwwLjQsMCwwLjUgYzAsMC4xLDAuMSwwLjIsMC4zLDAuMmMwLjEsMCwwLjIsMCwwLjMsMFY5Ljh6IE0xNS4yLDYuOWMtMS4yLDAuNi0zLjEsMC4yLTMuMSwxLjRjMCwxLjQsMy4xLDEsMy4xLTAuNVY2Ljl6Ii8+CiAgPC9nPgo8L3N2Zz4K);
  --jp-icon-check: url(data:image/svg+xml;base64,PHN2ZyB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIHdpZHRoPSIxNiIgdmlld0JveD0iMCAwIDI0IDI0Ij4KICA8ZyBjbGFzcz0ianAtaWNvbjMganAtaWNvbi1zZWxlY3RhYmxlIiBmaWxsPSIjNjE2MTYxIj4KICAgIDxwYXRoIGQ9Ik05IDE2LjE3TDQuODMgMTJsLTEuNDIgMS40MUw5IDE5IDIxIDdsLTEuNDEtMS40MXoiLz4KICA8L2c+Cjwvc3ZnPgo=);
  --jp-icon-circle-empty: url(data:image/svg+xml;base64,PHN2ZyB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIHdpZHRoPSIxNiIgdmlld0JveD0iMCAwIDI0IDI0Ij4KICA8ZyBjbGFzcz0ianAtaWNvbjMiIGZpbGw9IiM2MTYxNjEiPgogICAgPHBhdGggZD0iTTEyIDJDNi40NyAyIDIgNi40NyAyIDEyczQuNDcgMTAgMTAgMTAgMTAtNC40NyAxMC0xMFMxNy41MyAyIDEyIDJ6bTAgMThjLTQuNDEgMC04LTMuNTktOC04czMuNTktOCA4LTggOCAzLjU5IDggOC0zLjU5IDgtOCA4eiIvPgogIDwvZz4KPC9zdmc+Cg==);
  --jp-icon-circle: url(data:image/svg+xml;base64,PHN2ZyB2aWV3Qm94PSIwIDAgMTggMTgiIHdpZHRoPSIxNiIgeG1sbnM9Imh0dHA6Ly93d3cudzMub3JnLzIwMDAvc3ZnIj4KICA8ZyBjbGFzcz0ianAtaWNvbjMiIGZpbGw9IiM2MTYxNjEiPgogICAgPGNpcmNsZSBjeD0iOSIgY3k9IjkiIHI9IjgiLz4KICA8L2c+Cjwvc3ZnPgo=);
  --jp-icon-clear: url(data:image/svg+xml;base64,PHN2ZyB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIHdpZHRoPSIxNiIgdmlld0JveD0iMCAwIDI0IDI0Ij4KICA8bWFzayBpZD0iZG9udXRIb2xlIj4KICAgIDxyZWN0IHdpZHRoPSIyNCIgaGVpZ2h0PSIyNCIgZmlsbD0id2hpdGUiIC8+CiAgICA8Y2lyY2xlIGN4PSIxMiIgY3k9IjEyIiByPSI4IiBmaWxsPSJibGFjayIvPgogIDwvbWFzaz4KCiAgPGcgY2xhc3M9ImpwLWljb24zIiBmaWxsPSIjNjE2MTYxIj4KICAgIDxyZWN0IGhlaWdodD0iMTgiIHdpZHRoPSIyIiB4PSIxMSIgeT0iMyIgdHJhbnNmb3JtPSJyb3RhdGUoMzE1LCAxMiwgMTIpIi8+CiAgICA8Y2lyY2xlIGN4PSIxMiIgY3k9IjEyIiByPSIxMCIgbWFzaz0idXJsKCNkb251dEhvbGUpIi8+CiAgPC9nPgo8L3N2Zz4K);
  --jp-icon-close: url(data:image/svg+xml;base64,PHN2ZyB2aWV3Qm94PSIwIDAgMjQgMjQiIHdpZHRoPSIxNiIgeG1sbnM9Imh0dHA6Ly93d3cudzMub3JnLzIwMDAvc3ZnIj4KICA8ZyBjbGFzcz0ianAtaWNvbi1ub25lIGpwLWljb24tc2VsZWN0YWJsZS1pbnZlcnNlIGpwLWljb24zLWhvdmVyIiBmaWxsPSJub25lIj4KICAgIDxjaXJjbGUgY3g9IjEyIiBjeT0iMTIiIHI9IjExIi8+CiAgPC9nPgoKICA8ZyBjbGFzcz0ianAtaWNvbjMganAtaWNvbi1zZWxlY3RhYmxlIGpwLWljb24tYWNjZW50Mi1ob3ZlciIgZmlsbD0iIzYxNjE2MSI+CiAgICA8cGF0aCBkPSJNMTkgNi40MUwxNy41OSA1IDEyIDEwLjU5IDYuNDEgNSA1IDYuNDEgMTAuNTkgMTIgNSAxNy41OSA2LjQxIDE5IDEyIDEzLjQxIDE3LjU5IDE5IDE5IDE3LjU5IDEzLjQxIDEyeiIvPgogIDwvZz4KCiAgPGcgY2xhc3M9ImpwLWljb24tbm9uZSBqcC1pY29uLWJ1c3kiIGZpbGw9Im5vbmUiPgogICAgPGNpcmNsZSBjeD0iMTIiIGN5PSIxMiIgcj0iNyIvPgogIDwvZz4KPC9zdmc+Cg==);
  --jp-icon-code-check: url(data:image/svg+xml;base64,PHN2ZyB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIHdpZHRoPSIyNCIgdmlld0JveD0iMCAwIDI0IDI0Ij4KICA8ZyBjbGFzcz0ianAtaWNvbjMganAtaWNvbi1zZWxlY3RhYmxlIiBmaWxsPSIjNjE2MTYxIiBzaGFwZS1yZW5kZXJpbmc9Imdlb21ldHJpY1ByZWNpc2lvbiI+CiAgICA8cGF0aCBkPSJNNi41OSwzLjQxTDIsOEw2LjU5LDEyLjZMOCwxMS4xOEw0LjgyLDhMOCw0LjgyTDYuNTksMy40MU0xMi40MSwzLjQxTDExLDQuODJMMTQuMTgsOEwxMSwxMS4xOEwxMi40MSwxMi42TDE3LDhMMTIuNDEsMy40MU0yMS41OSwxMS41OUwxMy41LDE5LjY4TDkuODMsMTZMOC40MiwxNy40MUwxMy41LDIyLjVMMjMsMTNMMjEuNTksMTEuNTlaIiAvPgogIDwvZz4KPC9zdmc+Cg==);
  --jp-icon-code: url(data:image/svg+xml;base64,PHN2ZyB3aWR0aD0iMjIiIGhlaWdodD0iMjIiIHZpZXdCb3g9IjAgMCAyOCAyOCIgeG1sbnM9Imh0dHA6Ly93d3cudzMub3JnLzIwMDAvc3ZnIj4KCTxnIGNsYXNzPSJqcC1pY29uMyIgZmlsbD0iIzYxNjE2MSI+CgkJPHBhdGggZD0iTTExLjQgMTguNkw2LjggMTRMMTEuNCA5LjRMMTAgOEw0IDE0TDEwIDIwTDExLjQgMTguNlpNMTYuNiAxOC42TDIxLjIgMTRMMTYuNiA5LjRMMTggOEwyNCAxNEwxOCAyMEwxNi42IDE4LjZWMTguNloiLz4KCTwvZz4KPC9zdmc+Cg==);
  --jp-icon-collapse-all: url(data:image/svg+xml;base64,PHN2ZyB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIHdpZHRoPSIxNiIgdmlld0JveD0iMCAwIDI0IDI0Ij4KICAgIDxnIGNsYXNzPSJqcC1pY29uMyIgZmlsbD0iIzYxNjE2MSI+CiAgICAgICAgPHBhdGgKICAgICAgICAgICAgZD0iTTggMmMxIDAgMTEgMCAxMiAwczIgMSAyIDJjMCAxIDAgMTEgMCAxMnMwIDItMiAyQzIwIDE0IDIwIDQgMjAgNFMxMCA0IDYgNGMwLTIgMS0yIDItMnoiIC8+CiAgICAgICAgPHBhdGgKICAgICAgICAgICAgZD0iTTE4IDhjMC0xLTEtMi0yLTJTNSA2IDQgNnMtMiAxLTIgMmMwIDEgMCAxMSAwIDEyczEgMiAyIDJjMSAwIDExIDAgMTIgMHMyLTEgMi0yYzAtMSAwLTExIDAtMTJ6bS0yIDB2MTJINFY4eiIgLz4KICAgICAgICA8cGF0aCBkPSJNNiAxM3YyaDh2LTJ6IiAvPgogICAgPC9nPgo8L3N2Zz4K);
  --jp-icon-console: url(data:image/svg+xml;base64,PHN2ZyB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIHdpZHRoPSIxNiIgdmlld0JveD0iMCAwIDIwMCAyMDAiPgogIDxnIGNsYXNzPSJqcC1jb25zb2xlLWljb24tYmFja2dyb3VuZC1jb2xvciBqcC1pY29uLXNlbGVjdGFibGUiIGZpbGw9IiMwMjg4RDEiPgogICAgPHBhdGggZD0iTTIwIDE5LjhoMTYwdjE1OS45SDIweiIvPgogIDwvZz4KICA8ZyBjbGFzcz0ianAtY29uc29sZS1pY29uLWNvbG9yIGpwLWljb24tc2VsZWN0YWJsZS1pbnZlcnNlIiBmaWxsPSIjZmZmIj4KICAgIDxwYXRoIGQ9Ik0xMDUgMTI3LjNoNDB2MTIuOGgtNDB6TTUxLjEgNzdMNzQgOTkuOWwtMjMuMyAyMy4zIDEwLjUgMTAuNSAyMy4zLTIzLjNMOTUgOTkuOSA4NC41IDg5LjQgNjEuNiA2Ni41eiIvPgogIDwvZz4KPC9zdmc+Cg==);
  --jp-icon-copy: url(data:image/svg+xml;base64,PHN2ZyB2aWV3Qm94PSIwIDAgMTggMTgiIHdpZHRoPSIxNiIgeG1sbnM9Imh0dHA6Ly93d3cudzMub3JnLzIwMDAvc3ZnIj4KICA8ZyBjbGFzcz0ianAtaWNvbjMiIGZpbGw9IiM2MTYxNjEiPgogICAgPHBhdGggZD0iTTExLjksMUgzLjJDMi40LDEsMS43LDEuNywxLjcsMi41djEwLjJoMS41VjIuNWg4LjdWMXogTTE0LjEsMy45aC04Yy0wLjgsMC0xLjUsMC43LTEuNSwxLjV2MTAuMmMwLDAuOCwwLjcsMS41LDEuNSwxLjVoOCBjMC44LDAsMS41LTAuNywxLjUtMS41VjUuNEMxNS41LDQuNiwxNC45LDMuOSwxNC4xLDMuOXogTTE0LjEsMTUuNWgtOFY1LjRoOFYxNS41eiIvPgogIDwvZz4KPC9zdmc+Cg==);
  --jp-icon-copyright: url(data:image/svg+xml;base64,PHN2ZyB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIGVuYWJsZS1iYWNrZ3JvdW5kPSJuZXcgMCAwIDI0IDI0IiBoZWlnaHQ9IjI0IiB2aWV3Qm94PSIwIDAgMjQgMjQiIHdpZHRoPSIyNCI+CiAgPGcgY2xhc3M9ImpwLWljb24zIiBmaWxsPSIjNjE2MTYxIj4KICAgIDxwYXRoIGQ9Ik0xMS44OCw5LjE0YzEuMjgsMC4wNiwxLjYxLDEuMTUsMS42MywxLjY2aDEuNzljLTAuMDgtMS45OC0xLjQ5LTMuMTktMy40NS0zLjE5QzkuNjQsNy42MSw4LDksOCwxMi4xNCBjMCwxLjk0LDAuOTMsNC4yNCwzLjg0LDQuMjRjMi4yMiwwLDMuNDEtMS42NSwzLjQ0LTIuOTVoLTEuNzljLTAuMDMsMC41OS0wLjQ1LDEuMzgtMS42MywxLjQ0QzEwLjU1LDE0LjgzLDEwLDEzLjgxLDEwLDEyLjE0IEMxMCw5LjI1LDExLjI4LDkuMTYsMTEuODgsOS4xNHogTTEyLDJDNi40OCwyLDIsNi40OCwyLDEyczQuNDgsMTAsMTAsMTBzMTAtNC40OCwxMC0xMFMxNy41MiwyLDEyLDJ6IE0xMiwyMGMtNC40MSwwLTgtMy41OS04LTggczMuNTktOCw4LThzOCwzLjU5LDgsOFMxNi40MSwyMCwxMiwyMHoiLz4KICA8L2c+Cjwvc3ZnPgo=);
  --jp-icon-cut: url(data:image/svg+xml;base64,PHN2ZyB2aWV3Qm94PSIwIDAgMjQgMjQiIHdpZHRoPSIxNiIgeG1sbnM9Imh0dHA6Ly93d3cudzMub3JnLzIwMDAvc3ZnIj4KICA8ZyBjbGFzcz0ianAtaWNvbjMiIGZpbGw9IiM2MTYxNjEiPgogICAgPHBhdGggZD0iTTkuNjQgNy42NGMuMjMtLjUuMzYtMS4wNS4zNi0xLjY0IDAtMi4yMS0xLjc5LTQtNC00UzIgMy43OSAyIDZzMS43OSA0IDQgNGMuNTkgMCAxLjE0LS4xMyAxLjY0LS4zNkwxMCAxMmwtMi4zNiAyLjM2QzcuMTQgMTQuMTMgNi41OSAxNCA2IDE0Yy0yLjIxIDAtNCAxLjc5LTQgNHMxLjc5IDQgNCA0IDQtMS43OSA0LTRjMC0uNTktLjEzLTEuMTQtLjM2LTEuNjRMMTIgMTRsNyA3aDN2LTFMOS42NCA3LjY0ek02IDhjLTEuMSAwLTItLjg5LTItMnMuOS0yIDItMiAyIC44OSAyIDItLjkgMi0yIDJ6bTAgMTJjLTEuMSAwLTItLjg5LTItMnMuOS0yIDItMiAyIC44OSAyIDItLjkgMi0yIDJ6bTYtNy41Yy0uMjggMC0uNS0uMjItLjUtLjVzLjIyLS41LjUtLjUuNS4yMi41LjUtLjIyLjUtLjUuNXpNMTkgM2wtNiA2IDIgMiA3LTdWM3oiLz4KICA8L2c+Cjwvc3ZnPgo=);
  --jp-icon-delete: url(data:image/svg+xml;base64,PHN2ZyB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIHZpZXdCb3g9IjAgMCAyNCAyNCIgd2lkdGg9IjE2cHgiIGhlaWdodD0iMTZweCI+CiAgICA8cGF0aCBkPSJNMCAwaDI0djI0SDB6IiBmaWxsPSJub25lIiAvPgogICAgPHBhdGggY2xhc3M9ImpwLWljb24zIiBmaWxsPSIjNjI2MjYyIiBkPSJNNiAxOWMwIDEuMS45IDIgMiAyaDhjMS4xIDAgMi0uOSAyLTJWN0g2djEyek0xOSA0aC0zLjVsLTEtMWgtNWwtMSAxSDV2MmgxNFY0eiIgLz4KPC9zdmc+Cg==);
  --jp-icon-download: url(data:image/svg+xml;base64,PHN2ZyB2aWV3Qm94PSIwIDAgMjQgMjQiIHdpZHRoPSIxNiIgeG1sbnM9Imh0dHA6Ly93d3cudzMub3JnLzIwMDAvc3ZnIj4KICA8ZyBjbGFzcz0ianAtaWNvbjMiIGZpbGw9IiM2MTYxNjEiPgogICAgPHBhdGggZD0iTTE5IDloLTRWM0g5djZINWw3IDcgNy03ek01IDE4djJoMTR2LTJINXoiLz4KICA8L2c+Cjwvc3ZnPgo=);
  --jp-icon-duplicate: url(data:image/svg+xml;base64,PHN2ZyB3aWR0aD0iMTQiIGhlaWdodD0iMTQiIHZpZXdCb3g9IjAgMCAxNCAxNCIgZmlsbD0ibm9uZSIgeG1sbnM9Imh0dHA6Ly93d3cudzMub3JnLzIwMDAvc3ZnIj4KPHBhdGggY2xhc3M9ImpwLWljb24zIiBmaWxsLXJ1bGU9ImV2ZW5vZGQiIGNsaXAtcnVsZT0iZXZlbm9kZCIgZD0iTTIuNzk5OTggMC44NzVIOC44OTU4MkM5LjIwMDYxIDAuODc1IDkuNDQ5OTggMS4xMzkxNCA5LjQ0OTk4IDEuNDYxOThDOS40NDk5OCAxLjc4NDgyIDkuMjAwNjEgMi4wNDg5NiA4Ljg5NTgyIDIuMDQ4OTZIMy4zNTQxNUMzLjA0OTM2IDIuMDQ4OTYgMi43OTk5OCAyLjMxMzEgMi43OTk5OCAyLjYzNTk0VjkuNjc5NjlDMi43OTk5OCAxMC4wMDI1IDIuNTUwNjEgMTAuMjY2NyAyLjI0NTgyIDEwLjI2NjdDMS45NDEwMyAxMC4yNjY3IDEuNjkxNjUgMTAuMDAyNSAxLjY5MTY1IDkuNjc5NjlWMi4wNDg5NkMxLjY5MTY1IDEuNDAzMjggMi4xOTA0IDAuODc1IDIuNzk5OTggMC44NzVaTTUuMzY2NjUgMTEuOVY0LjU1SDExLjA4MzNWMTEuOUg1LjM2NjY1Wk00LjE0MTY1IDQuMTQxNjdDNC4xNDE2NSAzLjY5MDYzIDQuNTA3MjggMy4zMjUgNC45NTgzMiAzLjMyNUgxMS40OTE3QzExLjk0MjcgMy4zMjUgMTIuMzA4MyAzLjY5MDYzIDEyLjMwODMgNC4xNDE2N1YxMi4zMDgzQzEyLjMwODMgMTIuNzU5NCAxMS45NDI3IDEzLjEyNSAxMS40OTE3IDEzLjEyNUg0Ljk1ODMyQzQuNTA3MjggMTMuMTI1IDQuMTQxNjUgMTIuNzU5NCA0LjE0MTY1IDEyLjMwODNWNC4xNDE2N1oiIGZpbGw9IiM2MTYxNjEiLz4KPHBhdGggY2xhc3M9ImpwLWljb24zIiBkPSJNOS40MzU3NCA4LjI2NTA3SDguMzY0MzFWOS4zMzY1QzguMzY0MzEgOS40NTQzNSA4LjI2Nzg4IDkuNTUwNzggOC4xNTAwMiA5LjU1MDc4QzguMDMyMTcgOS41NTA3OCA3LjkzNTc0IDkuNDU0MzUgNy45MzU3NCA5LjMzNjVWOC4yNjUwN0g2Ljg2NDMxQzYuNzQ2NDUgOC4yNjUwNyA2LjY1MDAyIDguMTY4NjQgNi42NTAwMiA4LjA1MDc4QzYuNjUwMDIgNy45MzI5MiA2Ljc0NjQ1IDcuODM2NSA2Ljg2NDMxIDcuODM2NUg3LjkzNTc0VjYuNzY1MDdDNy45MzU3NCA2LjY0NzIxIDguMDMyMTcgNi41NTA3OCA4LjE1MDAyIDYuNTUwNzhDOC4yNjc4OCA2LjU1MDc4IDguMzY0MzEgNi42NDcyMSA4LjM2NDMxIDYuNzY1MDdWNy44MzY1SDkuNDM1NzRDOS41NTM2IDcuODM2NSA5LjY1MDAyIDcuOTMyOTIgOS42NTAwMiA4LjA1MDc4QzkuNjUwMDIgOC4xNjg2NCA5LjU1MzYgOC4yNjUwNyA5LjQzNTc0IDguMjY1MDdaIiBmaWxsPSIjNjE2MTYxIiBzdHJva2U9IiM2MTYxNjEiIHN0cm9rZS13aWR0aD0iMC41Ii8+Cjwvc3ZnPgo=);
  --jp-icon-edit: url(data:image/svg+xml;base64,PHN2ZyB2aWV3Qm94PSIwIDAgMjQgMjQiIHdpZHRoPSIxNiIgeG1sbnM9Imh0dHA6Ly93d3cudzMub3JnLzIwMDAvc3ZnIj4KICA8ZyBjbGFzcz0ianAtaWNvbjMiIGZpbGw9IiM2MTYxNjEiPgogICAgPHBhdGggZD0iTTMgMTcuMjVWMjFoMy43NUwxNy44MSA5Ljk0bC0zLjc1LTMuNzVMMyAxNy4yNXpNMjAuNzEgNy4wNGMuMzktLjM5LjM5LTEuMDIgMC0xLjQxbC0yLjM0LTIuMzRjLS4zOS0uMzktMS4wMi0uMzktMS40MSAwbC0xLjgzIDEuODMgMy43NSAzLjc1IDEuODMtMS44M3oiLz4KICA8L2c+Cjwvc3ZnPgo=);
  --jp-icon-ellipses: url(data:image/svg+xml;base64,PHN2ZyB2aWV3Qm94PSIwIDAgMjQgMjQiIHdpZHRoPSIxNiIgeG1sbnM9Imh0dHA6Ly93d3cudzMub3JnLzIwMDAvc3ZnIj4KICA8ZyBjbGFzcz0ianAtaWNvbjMiIGZpbGw9IiM2MTYxNjEiPgogICAgPGNpcmNsZSBjeD0iNSIgY3k9IjEyIiByPSIyIi8+CiAgICA8Y2lyY2xlIGN4PSIxMiIgY3k9IjEyIiByPSIyIi8+CiAgICA8Y2lyY2xlIGN4PSIxOSIgY3k9IjEyIiByPSIyIi8+CiAgPC9nPgo8L3N2Zz4K);
  --jp-icon-error: url(data:image/svg+xml;base64,PHN2ZyB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIHdpZHRoPSIxNiIgdmlld0JveD0iMCAwIDI0IDI0Ij4KPGcgY2xhc3M9ImpwLWljb24zIiBmaWxsPSIjNjE2MTYxIj48Y2lyY2xlIGN4PSIxMiIgY3k9IjE5IiByPSIyIi8+PHBhdGggZD0iTTEwIDNoNHYxMmgtNHoiLz48L2c+CjxwYXRoIGZpbGw9Im5vbmUiIGQ9Ik0wIDBoMjR2MjRIMHoiLz4KPC9zdmc+Cg==);
  --jp-icon-expand-all: url(data:image/svg+xml;base64,PHN2ZyB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIHdpZHRoPSIxNiIgdmlld0JveD0iMCAwIDI0IDI0Ij4KICAgIDxnIGNsYXNzPSJqcC1pY29uMyIgZmlsbD0iIzYxNjE2MSI+CiAgICAgICAgPHBhdGgKICAgICAgICAgICAgZD0iTTggMmMxIDAgMTEgMCAxMiAwczIgMSAyIDJjMCAxIDAgMTEgMCAxMnMwIDItMiAyQzIwIDE0IDIwIDQgMjAgNFMxMCA0IDYgNGMwLTIgMS0yIDItMnoiIC8+CiAgICAgICAgPHBhdGgKICAgICAgICAgICAgZD0iTTE4IDhjMC0xLTEtMi0yLTJTNSA2IDQgNnMtMiAxLTIgMmMwIDEgMCAxMSAwIDEyczEgMiAyIDJjMSAwIDExIDAgMTIgMHMyLTEgMi0yYzAtMSAwLTExIDAtMTJ6bS0yIDB2MTJINFY4eiIgLz4KICAgICAgICA8cGF0aCBkPSJNMTEgMTBIOXYzSDZ2MmgzdjNoMnYtM2gzdi0yaC0zeiIgLz4KICAgIDwvZz4KPC9zdmc+Cg==);
  --jp-icon-extension: url(data:image/svg+xml;base64,PHN2ZyB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIHdpZHRoPSIxNiIgdmlld0JveD0iMCAwIDI0IDI0Ij4KICA8ZyBjbGFzcz0ianAtaWNvbjMiIGZpbGw9IiM2MTYxNjEiPgogICAgPHBhdGggZD0iTTIwLjUgMTFIMTlWN2MwLTEuMS0uOS0yLTItMmgtNFYzLjVDMTMgMi4xMiAxMS44OCAxIDEwLjUgMVM4IDIuMTIgOCAzLjVWNUg0Yy0xLjEgMC0xLjk5LjktMS45OSAydjMuOEgzLjVjMS40OSAwIDIuNyAxLjIxIDIuNyAyLjdzLTEuMjEgMi43LTIuNyAyLjdIMlYyMGMwIDEuMS45IDIgMiAyaDMuOHYtMS41YzAtMS40OSAxLjIxLTIuNyAyLjctMi43IDEuNDkgMCAyLjcgMS4yMSAyLjcgMi43VjIySDE3YzEuMSAwIDItLjkgMi0ydi00aDEuNWMxLjM4IDAgMi41LTEuMTIgMi41LTIuNVMyMS44OCAxMSAyMC41IDExeiIvPgogIDwvZz4KPC9zdmc+Cg==);
  --jp-icon-fast-forward: url(data:image/svg+xml;base64,PHN2ZyB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIHdpZHRoPSIyNCIgaGVpZ2h0PSIyNCIgdmlld0JveD0iMCAwIDI0IDI0Ij4KICAgIDxnIGNsYXNzPSJqcC1pY29uMyIgZmlsbD0iIzYxNjE2MSI+CiAgICAgICAgPHBhdGggZD0iTTQgMThsOC41LTZMNCA2djEyem05LTEydjEybDguNS02TDEzIDZ6Ii8+CiAgICA8L2c+Cjwvc3ZnPgo=);
  --jp-icon-file-upload: url(data:image/svg+xml;base64,PHN2ZyB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIHdpZHRoPSIxNiIgdmlld0JveD0iMCAwIDI0IDI0Ij4KICA8ZyBjbGFzcz0ianAtaWNvbjMiIGZpbGw9IiM2MTYxNjEiPgogICAgPHBhdGggZD0iTTkgMTZoNnYtNmg0bC03LTctNyA3aDR6bS00IDJoMTR2Mkg1eiIvPgogIDwvZz4KPC9zdmc+Cg==);
  --jp-icon-file: url(data:image/svg+xml;base64,PHN2ZyB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIHdpZHRoPSIxNiIgdmlld0JveD0iMCAwIDIyIDIyIj4KICA8cGF0aCBjbGFzcz0ianAtaWNvbjMganAtaWNvbi1zZWxlY3RhYmxlIiBmaWxsPSIjNjE2MTYxIiBkPSJNMTkuMyA4LjJsLTUuNS01LjVjLS4zLS4zLS43LS41LTEuMi0uNUgzLjljLS44LjEtMS42LjktMS42IDEuOHYxNC4xYzAgLjkuNyAxLjYgMS42IDEuNmgxNC4yYy45IDAgMS42LS43IDEuNi0xLjZWOS40Yy4xLS41LS4xLS45LS40LTEuMnptLTUuOC0zLjNsMy40IDMuNmgtMy40VjQuOXptMy45IDEyLjdINC43Yy0uMSAwLS4yIDAtLjItLjJWNC43YzAtLjIuMS0uMy4yLS4zaDcuMnY0LjRzMCAuOC4zIDEuMWMuMy4zIDEuMS4zIDEuMS4zaDQuM3Y3LjJzLS4xLjItLjIuMnoiLz4KPC9zdmc+Cg==);
  --jp-icon-filter-dot: url(data:image/svg+xml;base64,PHN2ZyB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIHdpZHRoPSIxNiIgdmlld0JveD0iMCAwIDI0IDI0Ij4KICA8ZyBjbGFzcz0ianAtaWNvbjMiIGZpbGw9IiNGRkYiPgogICAgPHBhdGggZD0iTTE0LDEyVjE5Ljg4QzE0LjA0LDIwLjE4IDEzLjk0LDIwLjUgMTMuNzEsMjAuNzFDMTMuMzIsMjEuMSAxMi42OSwyMS4xIDEyLjMsMjAuNzFMMTAuMjksMTguN0MxMC4wNiwxOC40NyA5Ljk2LDE4LjE2IDEwLDE3Ljg3VjEySDkuOTdMNC4yMSw0LjYyQzMuODcsNC4xOSAzLjk1LDMuNTYgNC4zOCwzLjIyQzQuNTcsMy4wOCA0Ljc4LDMgNSwzVjNIMTlWM0MxOS4yMiwzIDE5LjQzLDMuMDggMTkuNjIsMy4yMkMyMC4wNSwzLjU2IDIwLjEzLDQuMTkgMTkuNzksNC42MkwxNC4wMywxMkgxNFoiIC8+CiAgPC9nPgogIDxnIGNsYXNzPSJqcC1pY29uLWRvdCIgZmlsbD0iI0ZGRiI+CiAgICA8Y2lyY2xlIGN4PSIxOCIgY3k9IjE3IiByPSIzIj48L2NpcmNsZT4KICA8L2c+Cjwvc3ZnPgo=);
  --jp-icon-filter-list: url(data:image/svg+xml;base64,PHN2ZyB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIHdpZHRoPSIxNiIgdmlld0JveD0iMCAwIDI0IDI0Ij4KICA8ZyBjbGFzcz0ianAtaWNvbjMiIGZpbGw9IiM2MTYxNjEiPgogICAgPHBhdGggZD0iTTEwIDE4aDR2LTJoLTR2MnpNMyA2djJoMThWNkgzem0zIDdoMTJ2LTJINnYyeiIvPgogIDwvZz4KPC9zdmc+Cg==);
  --jp-icon-filter: url(data:image/svg+xml;base64,PHN2ZyB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIHdpZHRoPSIxNiIgdmlld0JveD0iMCAwIDI0IDI0Ij4KICA8ZyBjbGFzcz0ianAtaWNvbjMiIGZpbGw9IiNGRkYiPgogICAgPHBhdGggZD0iTTE0LDEyVjE5Ljg4QzE0LjA0LDIwLjE4IDEzLjk0LDIwLjUgMTMuNzEsMjAuNzFDMTMuMzIsMjEuMSAxMi42OSwyMS4xIDEyLjMsMjAuNzFMMTAuMjksMTguN0MxMC4wNiwxOC40NyA5Ljk2LDE4LjE2IDEwLDE3Ljg3VjEySDkuOTdMNC4yMSw0LjYyQzMuODcsNC4xOSAzLjk1LDMuNTYgNC4zOCwzLjIyQzQuNTcsMy4wOCA0Ljc4LDMgNSwzVjNIMTlWM0MxOS4yMiwzIDE5LjQzLDMuMDggMTkuNjIsMy4yMkMyMC4wNSwzLjU2IDIwLjEzLDQuMTkgMTkuNzksNC42MkwxNC4wMywxMkgxNFoiIC8+CiAgPC9nPgo8L3N2Zz4K);
  --jp-icon-folder-favorite: url(data:image/svg+xml;base64,PHN2ZyB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIGhlaWdodD0iMjRweCIgdmlld0JveD0iMCAwIDI0IDI0IiB3aWR0aD0iMjRweCIgZmlsbD0iIzAwMDAwMCI+CiAgPHBhdGggZD0iTTAgMGgyNHYyNEgwVjB6IiBmaWxsPSJub25lIi8+PHBhdGggY2xhc3M9ImpwLWljb24zIGpwLWljb24tc2VsZWN0YWJsZSIgZmlsbD0iIzYxNjE2MSIgZD0iTTIwIDZoLThsLTItMkg0Yy0xLjEgMC0yIC45LTIgMnYxMmMwIDEuMS45IDIgMiAyaDE2YzEuMSAwIDItLjkgMi0yVjhjMC0xLjEtLjktMi0yLTJ6bS0yLjA2IDExTDE1IDE1LjI4IDEyLjA2IDE3bC43OC0zLjMzLTIuNTktMi4yNCAzLjQxLS4yOUwxNSA4bDEuMzQgMy4xNCAzLjQxLjI5LTIuNTkgMi4yNC43OCAzLjMzeiIvPgo8L3N2Zz4K);
  --jp-icon-folder: url(data:image/svg+xml;base64,PHN2ZyB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIHdpZHRoPSIxNiIgdmlld0JveD0iMCAwIDI0IDI0Ij4KICA8cGF0aCBjbGFzcz0ianAtaWNvbjMganAtaWNvbi1zZWxlY3RhYmxlIiBmaWxsPSIjNjE2MTYxIiBkPSJNMTAgNEg0Yy0xLjEgMC0xLjk5LjktMS45OSAyTDIgMThjMCAxLjEuOSAyIDIgMmgxNmMxLjEgMCAyLS45IDItMlY4YzAtMS4xLS45LTItMi0yaC04bC0yLTJ6Ii8+Cjwvc3ZnPgo=);
  --jp-icon-home: url(data:image/svg+xml;base64,PHN2ZyB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIGhlaWdodD0iMjRweCIgdmlld0JveD0iMCAwIDI0IDI0IiB3aWR0aD0iMjRweCIgZmlsbD0iIzAwMDAwMCI+CiAgPHBhdGggZD0iTTAgMGgyNHYyNEgweiIgZmlsbD0ibm9uZSIvPjxwYXRoIGNsYXNzPSJqcC1pY29uMyBqcC1pY29uLXNlbGVjdGFibGUiIGZpbGw9IiM2MTYxNjEiIGQ9Ik0xMCAyMHYtNmg0djZoNXYtOGgzTDEyIDMgMiAxMmgzdjh6Ii8+Cjwvc3ZnPgo=);
  --jp-icon-html5: url(data:image/svg+xml;base64,PHN2ZyB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIHdpZHRoPSIxNiIgdmlld0JveD0iMCAwIDUxMiA1MTIiPgogIDxwYXRoIGNsYXNzPSJqcC1pY29uMCBqcC1pY29uLXNlbGVjdGFibGUiIGZpbGw9IiMwMDAiIGQ9Ik0xMDguNCAwaDIzdjIyLjhoMjEuMlYwaDIzdjY5aC0yM1Y0NmgtMjF2MjNoLTIzLjJNMjA2IDIzaC0yMC4zVjBoNjMuN3YyM0gyMjl2NDZoLTIzbTUzLjUtNjloMjQuMWwxNC44IDI0LjNMMzEzLjIgMGgyNC4xdjY5aC0yM1YzNC44bC0xNi4xIDI0LjgtMTYuMS0yNC44VjY5aC0yMi42bTg5LjItNjloMjN2NDYuMmgzMi42VjY5aC01NS42Ii8+CiAgPHBhdGggY2xhc3M9ImpwLWljb24tc2VsZWN0YWJsZSIgZmlsbD0iI2U0NGQyNiIgZD0iTTEwNy42IDQ3MWwtMzMtMzcwLjRoMzYyLjhsLTMzIDM3MC4yTDI1NS43IDUxMiIvPgogIDxwYXRoIGNsYXNzPSJqcC1pY29uLXNlbGVjdGFibGUiIGZpbGw9IiNmMTY1MjkiIGQ9Ik0yNTYgNDgwLjVWMTMxaDE0OC4zTDM3NiA0NDciLz4KICA8cGF0aCBjbGFzcz0ianAtaWNvbi1zZWxlY3RhYmxlLWludmVyc2UiIGZpbGw9IiNlYmViZWIiIGQ9Ik0xNDIgMTc2LjNoMTE0djQ1LjRoLTY0LjJsNC4yIDQ2LjVoNjB2NDUuM0gxNTQuNG0yIDIyLjhIMjAybDMuMiAzNi4zIDUwLjggMTMuNnY0Ny40bC05My4yLTI2Ii8+CiAgPHBhdGggY2xhc3M9ImpwLWljb24tc2VsZWN0YWJsZS1pbnZlcnNlIiBmaWxsPSIjZmZmIiBkPSJNMzY5LjYgMTc2LjNIMjU1Ljh2NDUuNGgxMDkuNm0tNC4xIDQ2LjVIMjU1Ljh2NDUuNGg1NmwtNS4zIDU5LTUwLjcgMTMuNnY0Ny4ybDkzLTI1LjgiLz4KPC9zdmc+Cg==);
  --jp-icon-image: url(data:image/svg+xml;base64,PHN2ZyB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIHdpZHRoPSIxNiIgdmlld0JveD0iMCAwIDIyIDIyIj4KICA8cGF0aCBjbGFzcz0ianAtaWNvbi1icmFuZDQganAtaWNvbi1zZWxlY3RhYmxlLWludmVyc2UiIGZpbGw9IiNGRkYiIGQ9Ik0yLjIgMi4yaDE3LjV2MTcuNUgyLjJ6Ii8+CiAgPHBhdGggY2xhc3M9ImpwLWljb24tYnJhbmQwIGpwLWljb24tc2VsZWN0YWJsZSIgZmlsbD0iIzNGNTFCNSIgZD0iTTIuMiAyLjJ2MTcuNWgxNy41bC4xLTE3LjVIMi4yem0xMi4xIDIuMmMxLjIgMCAyLjIgMSAyLjIgMi4ycy0xIDIuMi0yLjIgMi4yLTIuMi0xLTIuMi0yLjIgMS0yLjIgMi4yLTIuMnpNNC40IDE3LjZsMy4zLTguOCAzLjMgNi42IDIuMi0zLjIgNC40IDUuNEg0LjR6Ii8+Cjwvc3ZnPgo=);
  --jp-icon-info: url(data:image/svg+xml;base64,PHN2ZyB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIHdpZHRoPSIxNiIgdmlld0JveD0iMCAwIDUwLjk3OCA1MC45NzgiPgoJPGcgY2xhc3M9ImpwLWljb24zIiBmaWxsPSIjNjE2MTYxIj4KCQk8cGF0aCBkPSJNNDMuNTIsNy40NThDMzguNzExLDIuNjQ4LDMyLjMwNywwLDI1LjQ4OSwwQzE4LjY3LDAsMTIuMjY2LDIuNjQ4LDcuNDU4LDcuNDU4CgkJCWMtOS45NDMsOS45NDEtOS45NDMsMjYuMTE5LDAsMzYuMDYyYzQuODA5LDQuODA5LDExLjIxMiw3LjQ1NiwxOC4wMzEsNy40NThjMCwwLDAuMDAxLDAsMC4wMDIsMAoJCQljNi44MTYsMCwxMy4yMjEtMi42NDgsMTguMDI5LTcuNDU4YzQuODA5LTQuODA5LDcuNDU3LTExLjIxMiw3LjQ1Ny0xOC4wM0M1MC45NzcsMTguNjcsNDguMzI4LDEyLjI2Niw0My41Miw3LjQ1OHoKCQkJIE00Mi4xMDYsNDIuMTA1Yy00LjQzMiw0LjQzMS0xMC4zMzIsNi44NzItMTYuNjE1LDYuODcyaC0wLjAwMmMtNi4yODUtMC4wMDEtMTIuMTg3LTIuNDQxLTE2LjYxNy02Ljg3MgoJCQljLTkuMTYyLTkuMTYzLTkuMTYyLTI0LjA3MSwwLTMzLjIzM0MxMy4zMDMsNC40NCwxOS4yMDQsMiwyNS40ODksMmM2LjI4NCwwLDEyLjE4NiwyLjQ0LDE2LjYxNyw2Ljg3MgoJCQljNC40MzEsNC40MzEsNi44NzEsMTAuMzMyLDYuODcxLDE2LjYxN0M0OC45NzcsMzEuNzcyLDQ2LjUzNiwzNy42NzUsNDIuMTA2LDQyLjEwNXoiLz4KCQk8cGF0aCBkPSJNMjMuNTc4LDMyLjIxOGMtMC4wMjMtMS43MzQsMC4xNDMtMy4wNTksMC40OTYtMy45NzJjMC4zNTMtMC45MTMsMS4xMS0xLjk5NywyLjI3Mi0zLjI1MwoJCQljMC40NjgtMC41MzYsMC45MjMtMS4wNjIsMS4zNjctMS41NzVjMC42MjYtMC43NTMsMS4xMDQtMS40NzgsMS40MzYtMi4xNzVjMC4zMzEtMC43MDcsMC40OTUtMS41NDEsMC40OTUtMi41CgkJCWMwLTEuMDk2LTAuMjYtMi4wODgtMC43NzktMi45NzljLTAuNTY1LTAuODc5LTEuNTAxLTEuMzM2LTIuODA2LTEuMzY5Yy0xLjgwMiwwLjA1Ny0yLjk4NSwwLjY2Ny0zLjU1LDEuODMyCgkJCWMtMC4zMDEsMC41MzUtMC41MDMsMS4xNDEtMC42MDcsMS44MTRjLTAuMTM5LDAuNzA3LTAuMjA3LDEuNDMyLTAuMjA3LDIuMTc0aC0yLjkzN2MtMC4wOTEtMi4yMDgsMC40MDctNC4xMTQsMS40OTMtNS43MTkKCQkJYzEuMDYyLTEuNjQsMi44NTUtMi40ODEsNS4zNzgtMi41MjdjMi4xNiwwLjAyMywzLjg3NCwwLjYwOCw1LjE0MSwxLjc1OGMxLjI3OCwxLjE2LDEuOTI5LDIuNzY0LDEuOTUsNC44MTEKCQkJYzAsMS4xNDItMC4xMzcsMi4xMTEtMC40MSwyLjkxMWMtMC4zMDksMC44NDUtMC43MzEsMS41OTMtMS4yNjgsMi4yNDNjLTAuNDkyLDAuNjUtMS4wNjgsMS4zMTgtMS43MywyLjAwMgoJCQljLTAuNjUsMC42OTctMS4zMTMsMS40NzktMS45ODcsMi4zNDZjLTAuMjM5LDAuMzc3LTAuNDI5LDAuNzc3LTAuNTY1LDEuMTk5Yy0wLjE2LDAuOTU5LTAuMjE3LDEuOTUxLTAuMTcxLDIuOTc5CgkJCUMyNi41ODksMzIuMjE4LDIzLjU3OCwzMi4yMTgsMjMuNTc4LDMyLjIxOHogTTIzLjU3OCwzOC4yMnYtMy40ODRoMy4wNzZ2My40ODRIMjMuNTc4eiIvPgoJPC9nPgo8L3N2Zz4K);
  --jp-icon-inspector: url(data:image/svg+xml;base64,PHN2ZyB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIHdpZHRoPSIxNiIgdmlld0JveD0iMCAwIDI0IDI0Ij4KICA8cGF0aCBjbGFzcz0ianAtaW5zcGVjdG9yLWljb24tY29sb3IganAtaWNvbi1zZWxlY3RhYmxlIiBmaWxsPSIjNjE2MTYxIiBkPSJNMjAgNEg0Yy0xLjEgMC0xLjk5LjktMS45OSAyTDIgMThjMCAxLjEuOSAyIDIgMmgxNmMxLjEgMCAyLS45IDItMlY2YzAtMS4xLS45LTItMi0yem0tNSAxNEg0di00aDExdjR6bTAtNUg0VjloMTF2NHptNSA1aC00VjloNHY5eiIvPgo8L3N2Zz4K);
  --jp-icon-json: url(data:image/svg+xml;base64,PHN2ZyB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIHdpZHRoPSIxNiIgdmlld0JveD0iMCAwIDIyIDIyIj4KICA8ZyBjbGFzcz0ianAtanNvbi1pY29uLWNvbG9yIGpwLWljb24tc2VsZWN0YWJsZSIgZmlsbD0iI0Y5QTgyNSI+CiAgICA8cGF0aCBkPSJNMjAuMiAxMS44Yy0xLjYgMC0xLjcuNS0xLjcgMSAwIC40LjEuOS4xIDEuMy4xLjUuMS45LjEgMS4zIDAgMS43LTEuNCAyLjMtMy41IDIuM2gtLjl2LTEuOWguNWMxLjEgMCAxLjQgMCAxLjQtLjggMC0uMyAwLS42LS4xLTEgMC0uNC0uMS0uOC0uMS0xLjIgMC0xLjMgMC0xLjggMS4zLTItMS4zLS4yLTEuMy0uNy0xLjMtMiAwLS40LjEtLjguMS0xLjIuMS0uNC4xLS43LjEtMSAwLS44LS40LS43LTEuNC0uOGgtLjVWNC4xaC45YzIuMiAwIDMuNS43IDMuNSAyLjMgMCAuNC0uMS45LS4xIDEuMy0uMS41LS4xLjktLjEgMS4zIDAgLjUuMiAxIDEuNyAxdjEuOHpNMS44IDEwLjFjMS42IDAgMS43LS41IDEuNy0xIDAtLjQtLjEtLjktLjEtMS4zLS4xLS41LS4xLS45LS4xLTEuMyAwLTEuNiAxLjQtMi4zIDMuNS0yLjNoLjl2MS45aC0uNWMtMSAwLTEuNCAwLTEuNC44IDAgLjMgMCAuNi4xIDEgMCAuMi4xLjYuMSAxIDAgMS4zIDAgMS44LTEuMyAyQzYgMTEuMiA2IDExLjcgNiAxM2MwIC40LS4xLjgtLjEgMS4yLS4xLjMtLjEuNy0uMSAxIDAgLjguMy44IDEuNC44aC41djEuOWgtLjljLTIuMSAwLTMuNS0uNi0zLjUtMi4zIDAtLjQuMS0uOS4xLTEuMy4xLS41LjEtLjkuMS0xLjMgMC0uNS0uMi0xLTEuNy0xdi0xLjl6Ii8+CiAgICA8Y2lyY2xlIGN4PSIxMSIgY3k9IjEzLjgiIHI9IjIuMSIvPgogICAgPGNpcmNsZSBjeD0iMTEiIGN5PSI4LjIiIHI9IjIuMSIvPgogIDwvZz4KPC9zdmc+Cg==);
  --jp-icon-julia: url(data:image/svg+xml;base64,PHN2ZyB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIHdpZHRoPSIxNiIgdmlld0JveD0iMCAwIDMyNSAzMDAiPgogIDxnIGNsYXNzPSJqcC1icmFuZDAganAtaWNvbi1zZWxlY3RhYmxlIiBmaWxsPSIjY2IzYzMzIj4KICAgIDxwYXRoIGQ9Ik0gMTUwLjg5ODQzOCAyMjUgQyAxNTAuODk4NDM4IDI2Ni40MjE4NzUgMTE3LjMyMDMxMiAzMDAgNzUuODk4NDM4IDMwMCBDIDM0LjQ3NjU2MiAzMDAgMC44OTg0MzggMjY2LjQyMTg3NSAwLjg5ODQzOCAyMjUgQyAwLjg5ODQzOCAxODMuNTc4MTI1IDM0LjQ3NjU2MiAxNTAgNzUuODk4NDM4IDE1MCBDIDExNy4zMjAzMTIgMTUwIDE1MC44OTg0MzggMTgzLjU3ODEyNSAxNTAuODk4NDM4IDIyNSIvPgogIDwvZz4KICA8ZyBjbGFzcz0ianAtYnJhbmQwIGpwLWljb24tc2VsZWN0YWJsZSIgZmlsbD0iIzM4OTgyNiI+CiAgICA8cGF0aCBkPSJNIDIzNy41IDc1IEMgMjM3LjUgMTE2LjQyMTg3NSAyMDMuOTIxODc1IDE1MCAxNjIuNSAxNTAgQyAxMjEuMDc4MTI1IDE1MCA4Ny41IDExNi40MjE4NzUgODcuNSA3NSBDIDg3LjUgMzMuNTc4MTI1IDEyMS4wNzgxMjUgMCAxNjIuNSAwIEMgMjAzLjkyMTg3NSAwIDIzNy41IDMzLjU3ODEyNSAyMzcuNSA3NSIvPgogIDwvZz4KICA8ZyBjbGFzcz0ianAtYnJhbmQwIGpwLWljb24tc2VsZWN0YWJsZSIgZmlsbD0iIzk1NThiMiI+CiAgICA8cGF0aCBkPSJNIDMyNC4xMDE1NjIgMjI1IEMgMzI0LjEwMTU2MiAyNjYuNDIxODc1IDI5MC41MjM0MzggMzAwIDI0OS4xMDE1NjIgMzAwIEMgMjA3LjY3OTY4OCAzMDAgMTc0LjEwMTU2MiAyNjYuNDIxODc1IDE3NC4xMDE1NjIgMjI1IEMgMTc0LjEwMTU2MiAxODMuNTc4MTI1IDIwNy42Nzk2ODggMTUwIDI0OS4xMDE1NjIgMTUwIEMgMjkwLjUyMzQzOCAxNTAgMzI0LjEwMTU2MiAxODMuNTc4MTI1IDMyNC4xMDE1NjIgMjI1Ii8+CiAgPC9nPgo8L3N2Zz4K);
  --jp-icon-jupyter-favicon: url(data:image/svg+xml;base64,PHN2ZyB3aWR0aD0iMTUyIiBoZWlnaHQ9IjE2NSIgdmlld0JveD0iMCAwIDE1MiAxNjUiIHZlcnNpb249IjEuMSIgeG1sbnM9Imh0dHA6Ly93d3cudzMub3JnLzIwMDAvc3ZnIj4KICAgPGcgY2xhc3M9ImpwLWp1cHl0ZXItaWNvbi1jb2xvciIgZmlsbD0iI0YzNzcyNiI+CiAgICA8cGF0aCB0cmFuc2Zvcm09InRyYW5zbGF0ZSgwLjA3ODk0NywgMTEwLjU4MjkyNykiIGQ9Ik03NS45NDIyODQyLDI5LjU4MDQ1NjEgQzQzLjMwMjM5NDcsMjkuNTgwNDU2MSAxNC43OTY3ODMyLDE3LjY1MzQ2MzQgMCwwIEM1LjUxMDgzMjExLDE1Ljg0MDY4MjkgMTUuNzgxNTM4OSwyOS41NjY3NzMyIDI5LjM5MDQ5NDcsMzkuMjc4NDE3MSBDNDIuOTk5Nyw0OC45ODk4NTM3IDU5LjI3MzcsNTQuMjA2NzgwNSA3NS45NjA1Nzg5LDU0LjIwNjc4MDUgQzkyLjY0NzQ1NzksNTQuMjA2NzgwNSAxMDguOTIxNDU4LDQ4Ljk4OTg1MzcgMTIyLjUzMDY2MywzOS4yNzg0MTcxIEMxMzYuMTM5NDUzLDI5LjU2Njc3MzIgMTQ2LjQxMDI4NCwxNS44NDA2ODI5IDE1MS45MjExNTgsMCBDMTM3LjA4Nzg2OCwxNy42NTM0NjM0IDEwOC41ODI1ODksMjkuNTgwNDU2MSA3NS45NDIyODQyLDI5LjU4MDQ1NjEgTDc1Ljk0MjI4NDIsMjkuNTgwNDU2MSBaIiAvPgogICAgPHBhdGggdHJhbnNmb3JtPSJ0cmFuc2xhdGUoMC4wMzczNjgsIDAuNzA0ODc4KSIgZD0iTTc1Ljk3ODQ1NzksMjQuNjI2NDA3MyBDMTA4LjYxODc2MywyNC42MjY0MDczIDEzNy4xMjQ0NTgsMzYuNTUzNDQxNSAxNTEuOTIxMTU4LDU0LjIwNjc4MDUgQzE0Ni40MTAyODQsMzguMzY2MjIyIDEzNi4xMzk0NTMsMjQuNjQwMTMxNyAxMjIuNTMwNjYzLDE0LjkyODQ4NzggQzEwOC45MjE0NTgsNS4yMTY4NDM5IDkyLjY0NzQ1NzksMCA3NS45NjA1Nzg5LDAgQzU5LjI3MzcsMCA0Mi45OTk3LDUuMjE2ODQzOSAyOS4zOTA0OTQ3LDE0LjkyODQ4NzggQzE1Ljc4MTUzODksMjQuNjQwMTMxNyA1LjUxMDgzMjExLDM4LjM2NjIyMiAwLDU0LjIwNjc4MDUgQzE0LjgzMzA4MTYsMzYuNTg5OTI5MyA0My4zMzg1Njg0LDI0LjYyNjQwNzMgNzUuOTc4NDU3OSwyNC42MjY0MDczIEw3NS45Nzg0NTc5LDI0LjYyNjQwNzMgWiIgLz4KICA8L2c+Cjwvc3ZnPgo=);
  --jp-icon-jupyter: url(data:image/svg+xml;base64,PHN2ZyB3aWR0aD0iMzkiIGhlaWdodD0iNTEiIHZpZXdCb3g9IjAgMCAzOSA1MSIgeG1sbnM9Imh0dHA6Ly93d3cudzMub3JnLzIwMDAvc3ZnIj4KICA8ZyB0cmFuc2Zvcm09InRyYW5zbGF0ZSgtMTYzOCAtMjI4MSkiPgogICAgIDxnIGNsYXNzPSJqcC1qdXB5dGVyLWljb24tY29sb3IiIGZpbGw9IiNGMzc3MjYiPgogICAgICA8cGF0aCB0cmFuc2Zvcm09InRyYW5zbGF0ZSgxNjM5Ljc0IDIzMTEuOTgpIiBkPSJNIDE4LjI2NDYgNy4xMzQxMUMgMTAuNDE0NSA3LjEzNDExIDMuNTU4NzIgNC4yNTc2IDAgMEMgMS4zMjUzOSAzLjgyMDQgMy43OTU1NiA3LjEzMDgxIDcuMDY4NiA5LjQ3MzAzQyAxMC4zNDE3IDExLjgxNTIgMTQuMjU1NyAxMy4wNzM0IDE4LjI2OSAxMy4wNzM0QyAyMi4yODIzIDEzLjA3MzQgMjYuMTk2MyAxMS44MTUyIDI5LjQ2OTQgOS40NzMwM0MgMzIuNzQyNCA3LjEzMDgxIDM1LjIxMjYgMy44MjA0IDM2LjUzOCAwQyAzMi45NzA1IDQuMjU3NiAyNi4xMTQ4IDcuMTM0MTEgMTguMjY0NiA3LjEzNDExWiIvPgogICAgICA8cGF0aCB0cmFuc2Zvcm09InRyYW5zbGF0ZSgxNjM5LjczIDIyODUuNDgpIiBkPSJNIDE4LjI3MzMgNS45MzkzMUMgMjYuMTIzNSA1LjkzOTMxIDMyLjk3OTMgOC44MTU4MyAzNi41MzggMTMuMDczNEMgMzUuMjEyNiA5LjI1MzAzIDMyLjc0MjQgNS45NDI2MiAyOS40Njk0IDMuNjAwNEMgMjYuMTk2MyAxLjI1ODE4IDIyLjI4MjMgMCAxOC4yNjkgMEMgMTQuMjU1NyAwIDEwLjM0MTcgMS4yNTgxOCA3LjA2ODYgMy42MDA0QyAzLjc5NTU2IDUuOTQyNjIgMS4zMjUzOSA5LjI1MzAzIDAgMTMuMDczNEMgMy41Njc0NSA4LjgyNDYzIDEwLjQyMzIgNS45MzkzMSAxOC4yNzMzIDUuOTM5MzFaIi8+CiAgICA8L2c+CiAgICA8ZyBjbGFzcz0ianAtaWNvbjMiIGZpbGw9IiM2MTYxNjEiPgogICAgICA8cGF0aCB0cmFuc2Zvcm09InRyYW5zbGF0ZSgxNjY5LjMgMjI4MS4zMSkiIGQ9Ik0gNS44OTM1MyAyLjg0NEMgNS45MTg4OSAzLjQzMTY1IDUuNzcwODUgNC4wMTM2NyA1LjQ2ODE1IDQuNTE2NDVDIDUuMTY1NDUgNS4wMTkyMiA0LjcyMTY4IDUuNDIwMTUgNC4xOTI5OSA1LjY2ODUxQyAzLjY2NDMgNS45MTY4OCAzLjA3NDQ0IDYuMDAxNTEgMi40OTgwNSA1LjkxMTcxQyAxLjkyMTY2IDUuODIxOSAxLjM4NDYzIDUuNTYxNyAwLjk1NDg5OCA1LjE2NDAxQyAwLjUyNTE3IDQuNzY2MzMgMC4yMjIwNTYgNC4yNDkwMyAwLjA4MzkwMzcgMy42Nzc1N0MgLTAuMDU0MjQ4MyAzLjEwNjExIC0wLjAyMTIzIDIuNTA2MTcgMC4xNzg3ODEgMS45NTM2NEMgMC4zNzg3OTMgMS40MDExIDAuNzM2ODA5IDAuOTIwODE3IDEuMjA3NTQgMC41NzM1MzhDIDEuNjc4MjYgMC4yMjYyNTkgMi4yNDA1NSAwLjAyNzU5MTkgMi44MjMyNiAwLjAwMjY3MjI5QyAzLjYwMzg5IC0wLjAzMDcxMTUgNC4zNjU3MyAwLjI0OTc4OSA0Ljk0MTQyIDAuNzgyNTUxQyA1LjUxNzExIDEuMzE1MzEgNS44NTk1NiAyLjA1Njc2IDUuODkzNTMgMi44NDRaIi8+CiAgICAgIDxwYXRoIHRyYW5zZm9ybT0idHJhbnNsYXRlKDE2MzkuOCAyMzIzLjgxKSIgZD0iTSA3LjQyNzg5IDMuNTgzMzhDIDcuNDYwMDggNC4zMjQzIDcuMjczNTUgNS4wNTgxOSA2Ljg5MTkzIDUuNjkyMTNDIDYuNTEwMzEgNi4zMjYwNyA1Ljk1MDc1IDYuODMxNTYgNS4yODQxMSA3LjE0NDZDIDQuNjE3NDcgNy40NTc2MyAzLjg3MzcxIDcuNTY0MTQgMy4xNDcwMiA3LjQ1MDYzQyAyLjQyMDMyIDcuMzM3MTIgMS43NDMzNiA3LjAwODcgMS4yMDE4NCA2LjUwNjk1QyAwLjY2MDMyOCA2LjAwNTIgMC4yNzg2MSA1LjM1MjY4IDAuMTA1MDE3IDQuNjMyMDJDIC0wLjA2ODU3NTcgMy45MTEzNSAtMC4wMjYyMzYxIDMuMTU0OTQgMC4yMjY2NzUgMi40NTg1NkMgMC40Nzk1ODcgMS43NjIxNyAwLjkzMTY5NyAxLjE1NzEzIDEuNTI1NzYgMC43MjAwMzNDIDIuMTE5ODMgMC4yODI5MzUgMi44MjkxNCAwLjAzMzQzOTUgMy41NjM4OSAwLjAwMzEzMzQ0QyA0LjU0NjY3IC0wLjAzNzQwMzMgNS41MDUyOSAwLjMxNjcwNiA2LjIyOTYxIDAuOTg3ODM1QyA2Ljk1MzkzIDEuNjU4OTYgNy4zODQ4NCAyLjU5MjM1IDcuNDI3ODkgMy41ODMzOEwgNy40Mjc4OSAzLjU4MzM4WiIvPgogICAgICA8cGF0aCB0cmFuc2Zvcm09InRyYW5zbGF0ZSgxNjM4LjM2IDIyODYuMDYpIiBkPSJNIDIuMjc0NzEgNC4zOTYyOUMgMS44NDM2MyA0LjQxNTA4IDEuNDE2NzEgNC4zMDQ0NSAxLjA0Nzk5IDQuMDc4NDNDIDAuNjc5MjY4IDMuODUyNCAwLjM4NTMyOCAzLjUyMTE0IDAuMjAzMzcxIDMuMTI2NTZDIDAuMDIxNDEzNiAyLjczMTk4IC0wLjA0MDM3OTggMi4yOTE4MyAwLjAyNTgxMTYgMS44NjE4MUMgMC4wOTIwMDMxIDEuNDMxOCAwLjI4MzIwNCAxLjAzMTI2IDAuNTc1MjEzIDAuNzEwODgzQyAwLjg2NzIyMiAwLjM5MDUxIDEuMjQ2OTEgMC4xNjQ3MDggMS42NjYyMiAwLjA2MjA1OTJDIDIuMDg1NTMgLTAuMDQwNTg5NyAyLjUyNTYxIC0wLjAxNTQ3MTQgMi45MzA3NiAwLjEzNDIzNUMgMy4zMzU5MSAwLjI4Mzk0MSAzLjY4NzkyIDAuNTUxNTA1IDMuOTQyMjIgMC45MDMwNkMgNC4xOTY1MiAxLjI1NDYyIDQuMzQxNjkgMS42NzQzNiA0LjM1OTM1IDIuMTA5MTZDIDQuMzgyOTkgMi42OTEwNyA0LjE3Njc4IDMuMjU4NjkgMy43ODU5NyAzLjY4NzQ2QyAzLjM5NTE2IDQuMTE2MjQgMi44NTE2NiA0LjM3MTE2IDIuMjc0NzEgNC4zOTYyOUwgMi4yNzQ3MSA0LjM5NjI5WiIvPgogICAgPC9nPgogIDwvZz4+Cjwvc3ZnPgo=);
  --jp-icon-jupyterlab-wordmark: url(data:image/svg+xml;base64,PHN2ZyB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIHdpZHRoPSIyMDAiIHZpZXdCb3g9IjAgMCAxODYwLjggNDc1Ij4KICA8ZyBjbGFzcz0ianAtaWNvbjIiIGZpbGw9IiM0RTRFNEUiIHRyYW5zZm9ybT0idHJhbnNsYXRlKDQ4MC4xMzY0MDEsIDY0LjI3MTQ5MykiPgogICAgPGcgdHJhbnNmb3JtPSJ0cmFuc2xhdGUoMC4wMDAwMDAsIDU4Ljg3NTU2NikiPgogICAgICA8ZyB0cmFuc2Zvcm09InRyYW5zbGF0ZSgwLjA4NzYwMywgMC4xNDAyOTQpIj4KICAgICAgICA8cGF0aCBkPSJNLTQyNi45LDE2OS44YzAsNDguNy0zLjcsNjQuNy0xMy42LDc2LjRjLTEwLjgsMTAtMjUsMTUuNS0zOS43LDE1LjVsMy43LDI5IGMyMi44LDAuMyw0NC44LTcuOSw2MS45LTIzLjFjMTcuOC0xOC41LDI0LTQ0LjEsMjQtODMuM1YwSC00Mjd2MTcwLjFMLTQyNi45LDE2OS44TC00MjYuOSwxNjkuOHoiLz4KICAgICAgPC9nPgogICAgPC9nPgogICAgPGcgdHJhbnNmb3JtPSJ0cmFuc2xhdGUoMTU1LjA0NTI5NiwgNTYuODM3MTA0KSI+CiAgICAgIDxnIHRyYW5zZm9ybT0idHJhbnNsYXRlKDEuNTYyNDUzLCAxLjc5OTg0MikiPgogICAgICAgIDxwYXRoIGQ9Ik0tMzEyLDE0OGMwLDIxLDAsMzkuNSwxLjcsNTUuNGgtMzEuOGwtMi4xLTMzLjNoLTAuOGMtNi43LDExLjYtMTYuNCwyMS4zLTI4LDI3LjkgYy0xMS42LDYuNi0yNC44LDEwLTM4LjIsOS44Yy0zMS40LDAtNjktMTcuNy02OS04OVYwaDM2LjR2MTEyLjdjMCwzOC43LDExLjYsNjQuNyw0NC42LDY0LjdjMTAuMy0wLjIsMjAuNC0zLjUsMjguOS05LjQgYzguNS01LjksMTUuMS0xNC4zLDE4LjktMjMuOWMyLjItNi4xLDMuMy0xMi41LDMuMy0xOC45VjAuMmgzNi40VjE0OEgtMzEyTC0zMTIsMTQ4eiIvPgogICAgICA8L2c+CiAgICA8L2c+CiAgICA8ZyB0cmFuc2Zvcm09InRyYW5zbGF0ZSgzOTAuMDEzMzIyLCA1My40Nzk2MzgpIj4KICAgICAgPGcgdHJhbnNmb3JtPSJ0cmFuc2xhdGUoMS43MDY0NTgsIDAuMjMxNDI1KSI+CiAgICAgICAgPHBhdGggZD0iTS00NzguNiw3MS40YzAtMjYtMC44LTQ3LTEuNy02Ni43aDMyLjdsMS43LDM0LjhoMC44YzcuMS0xMi41LDE3LjUtMjIuOCwzMC4xLTI5LjcgYzEyLjUtNywyNi43LTEwLjMsNDEtOS44YzQ4LjMsMCw4NC43LDQxLjcsODQuNywxMDMuM2MwLDczLjEtNDMuNywxMDkuMi05MSwxMDkuMmMtMTIuMSwwLjUtMjQuMi0yLjItMzUtNy44IGMtMTAuOC01LjYtMTkuOS0xMy45LTI2LjYtMjQuMmgtMC44VjI5MWgtMzZ2LTIyMEwtNDc4LjYsNzEuNEwtNDc4LjYsNzEuNHogTS00NDIuNiwxMjUuNmMwLjEsNS4xLDAuNiwxMC4xLDEuNywxNS4xIGMzLDEyLjMsOS45LDIzLjMsMTkuOCwzMS4xYzkuOSw3LjgsMjIuMSwxMi4xLDM0LjcsMTIuMWMzOC41LDAsNjAuNy0zMS45LDYwLjctNzguNWMwLTQwLjctMjEuMS03NS42LTU5LjUtNzUuNiBjLTEyLjksMC40LTI1LjMsNS4xLTM1LjMsMTMuNGMtOS45LDguMy0xNi45LDE5LjctMTkuNiwzMi40Yy0xLjUsNC45LTIuMywxMC0yLjUsMTUuMVYxMjUuNkwtNDQyLjYsMTI1LjZMLTQ0Mi42LDEyNS42eiIvPgogICAgICA8L2c+CiAgICA8L2c+CiAgICA8ZyB0cmFuc2Zvcm09InRyYW5zbGF0ZSg2MDYuNzQwNzI2LCA1Ni44MzcxMDQpIj4KICAgICAgPGcgdHJhbnNmb3JtPSJ0cmFuc2xhdGUoMC43NTEyMjYsIDEuOTg5Mjk5KSI+CiAgICAgICAgPHBhdGggZD0iTS00NDAuOCwwbDQzLjcsMTIwLjFjNC41LDEzLjQsOS41LDI5LjQsMTIuOCw0MS43aDAuOGMzLjctMTIuMiw3LjktMjcuNywxMi44LTQyLjQgbDM5LjctMTE5LjJoMzguNUwtMzQ2LjksMTQ1Yy0yNiw2OS43LTQzLjcsMTA1LjQtNjguNiwxMjcuMmMtMTIuNSwxMS43LTI3LjksMjAtNDQuNiwyMy45bC05LjEtMzEuMSBjMTEuNy0zLjksMjIuNS0xMC4xLDMxLjgtMTguMWMxMy4yLTExLjEsMjMuNy0yNS4yLDMwLjYtNDEuMmMxLjUtMi44LDIuNS01LjcsMi45LTguOGMtMC4zLTMuMy0xLjItNi42LTIuNS05LjdMLTQ4MC4yLDAuMSBoMzkuN0wtNDQwLjgsMEwtNDQwLjgsMHoiLz4KICAgICAgPC9nPgogICAgPC9nPgogICAgPGcgdHJhbnNmb3JtPSJ0cmFuc2xhdGUoODIyLjc0ODEwNCwgMC4wMDAwMDApIj4KICAgICAgPGcgdHJhbnNmb3JtPSJ0cmFuc2xhdGUoMS40NjQwNTAsIDAuMzc4OTE0KSI+CiAgICAgICAgPHBhdGggZD0iTS00MTMuNywwdjU4LjNoNTJ2MjguMmgtNTJWMTk2YzAsMjUsNywzOS41LDI3LjMsMzkuNWM3LjEsMC4xLDE0LjItMC43LDIxLjEtMi41IGwxLjcsMjcuN2MtMTAuMywzLjctMjEuMyw1LjQtMzIuMiw1Yy03LjMsMC40LTE0LjYtMC43LTIxLjMtMy40Yy02LjgtMi43LTEyLjktNi44LTE3LjktMTIuMWMtMTAuMy0xMC45LTE0LjEtMjktMTQuMS01Mi45IFY4Ni41aC0zMVY1OC4zaDMxVjkuNkwtNDEzLjcsMEwtNDEzLjcsMHoiLz4KICAgICAgPC9nPgogICAgPC9nPgogICAgPGcgdHJhbnNmb3JtPSJ0cmFuc2xhdGUoOTc0LjQzMzI4NiwgNTMuNDc5NjM4KSI+CiAgICAgIDxnIHRyYW5zZm9ybT0idHJhbnNsYXRlKDAuOTkwMDM0LCAwLjYxMDMzOSkiPgogICAgICAgIDxwYXRoIGQ9Ik0tNDQ1LjgsMTEzYzAuOCw1MCwzMi4yLDcwLjYsNjguNiw3MC42YzE5LDAuNiwzNy45LTMsNTUuMy0xMC41bDYuMiwyNi40IGMtMjAuOSw4LjktNDMuNSwxMy4xLTY2LjIsMTIuNmMtNjEuNSwwLTk4LjMtNDEuMi05OC4zLTEwMi41Qy00ODAuMiw0OC4yLTQ0NC43LDAtMzg2LjUsMGM2NS4yLDAsODIuNyw1OC4zLDgyLjcsOTUuNyBjLTAuMSw1LjgtMC41LDExLjUtMS4yLDE3LjJoLTE0MC42SC00NDUuOEwtNDQ1LjgsMTEzeiBNLTMzOS4yLDg2LjZjMC40LTIzLjUtOS41LTYwLjEtNTAuNC02MC4xIGMtMzYuOCwwLTUyLjgsMzQuNC01NS43LDYwLjFILTMzOS4yTC0zMzkuMiw4Ni42TC0zMzkuMiw4Ni42eiIvPgogICAgICA8L2c+CiAgICA8L2c+CiAgICA8ZyB0cmFuc2Zvcm09InRyYW5zbGF0ZSgxMjAxLjk2MTA1OCwgNTMuNDc5NjM4KSI+CiAgICAgIDxnIHRyYW5zZm9ybT0idHJhbnNsYXRlKDEuMTc5NjQwLCAwLjcwNTA2OCkiPgogICAgICAgIDxwYXRoIGQ9Ik0tNDc4LjYsNjhjMC0yMy45LTAuNC00NC41LTEuNy02My40aDMxLjhsMS4yLDM5LjloMS43YzkuMS0yNy4zLDMxLTQ0LjUsNTUuMy00NC41IGMzLjUtMC4xLDcsMC40LDEwLjMsMS4ydjM0LjhjLTQuMS0wLjktOC4yLTEuMy0xMi40LTEuMmMtMjUuNiwwLTQzLjcsMTkuNy00OC43LDQ3LjRjLTEsNS43LTEuNiwxMS41LTEuNywxNy4ydjEwOC4zaC0zNlY2OCBMLTQ3OC42LDY4eiIvPgogICAgICA8L2c+CiAgICA8L2c+CiAgPC9nPgoKICA8ZyBjbGFzcz0ianAtaWNvbi13YXJuMCIgZmlsbD0iI0YzNzcyNiI+CiAgICA8cGF0aCBkPSJNMTM1Mi4zLDMyNi4yaDM3VjI4aC0zN1YzMjYuMnogTTE2MDQuOCwzMjYuMmMtMi41LTEzLjktMy40LTMxLjEtMy40LTQ4Ljd2LTc2IGMwLTQwLjctMTUuMS04My4xLTc3LjMtODMuMWMtMjUuNiwwLTUwLDcuMS02Ni44LDE4LjFsOC40LDI0LjRjMTQuMy05LjIsMzQtMTUuMSw1My0xNS4xYzQxLjYsMCw0Ni4yLDMwLjIsNDYuMiw0N3Y0LjIgYy03OC42LTAuNC0xMjIuMywyNi41LTEyMi4zLDc1LjZjMCwyOS40LDIxLDU4LjQsNjIuMiw1OC40YzI5LDAsNTAuOS0xNC4zLDYyLjItMzAuMmgxLjNsMi45LDI1LjZIMTYwNC44eiBNMTU2NS43LDI1Ny43IGMwLDMuOC0wLjgsOC0yLjEsMTEuOGMtNS45LDE3LjItMjIuNywzNC00OS4yLDM0Yy0xOC45LDAtMzQuOS0xMS4zLTM0LjktMzUuM2MwLTM5LjUsNDUuOC00Ni42LDg2LjItNDUuOFYyNTcuN3ogTTE2OTguNSwzMjYuMiBsMS43LTMzLjZoMS4zYzE1LjEsMjYuOSwzOC43LDM4LjIsNjguMSwzOC4yYzQ1LjQsMCw5MS4yLTM2LjEsOTEuMi0xMDguOGMwLjQtNjEuNy0zNS4zLTEwMy43LTg1LjctMTAzLjcgYy0zMi44LDAtNTYuMywxNC43LTY5LjMsMzcuNGgtMC44VjI4aC0zNi42djI0NS43YzAsMTguMS0wLjgsMzguNi0xLjcsNTIuNUgxNjk4LjV6IE0xNzA0LjgsMjA4LjJjMC01LjksMS4zLTEwLjksMi4xLTE1LjEgYzcuNi0yOC4xLDMxLjEtNDUuNCw1Ni4zLTQ1LjRjMzkuNSwwLDYwLjUsMzQuOSw2MC41LDc1LjZjMCw0Ni42LTIzLjEsNzguMS02MS44LDc4LjFjLTI2LjksMC00OC4zLTE3LjYtNTUuNS00My4zIGMtMC44LTQuMi0xLjctOC44LTEuNy0xMy40VjIwOC4yeiIvPgogIDwvZz4KPC9zdmc+Cg==);
  --jp-icon-kernel: url(data:image/svg+xml;base64,PHN2ZyB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIHdpZHRoPSIxNiIgdmlld0JveD0iMCAwIDI0IDI0Ij4KICAgIDxwYXRoIGNsYXNzPSJqcC1pY29uMiIgZmlsbD0iIzYxNjE2MSIgZD0iTTE1IDlIOXY2aDZWOXptLTIgNGgtMnYtMmgydjJ6bTgtMlY5aC0yVjdjMC0xLjEtLjktMi0yLTJoLTJWM2gtMnYyaC0yVjNIOXYySDdjLTEuMSAwLTIgLjktMiAydjJIM3YyaDJ2MkgzdjJoMnYyYzAgMS4xLjkgMiAyIDJoMnYyaDJ2LTJoMnYyaDJ2LTJoMmMxLjEgMCAyLS45IDItMnYtMmgydi0yaC0ydi0yaDJ6bS00IDZIN1Y3aDEwdjEweiIvPgo8L3N2Zz4K);
  --jp-icon-keyboard: url(data:image/svg+xml;base64,PHN2ZyB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIHdpZHRoPSIxNiIgdmlld0JveD0iMCAwIDI0IDI0Ij4KICA8cGF0aCBjbGFzcz0ianAtaWNvbjMganAtaWNvbi1zZWxlY3RhYmxlIiBmaWxsPSIjNjE2MTYxIiBkPSJNMjAgNUg0Yy0xLjEgMC0xLjk5LjktMS45OSAyTDIgMTdjMCAxLjEuOSAyIDIgMmgxNmMxLjEgMCAyLS45IDItMlY3YzAtMS4xLS45LTItMi0yem0tOSAzaDJ2MmgtMlY4em0wIDNoMnYyaC0ydi0yek04IDhoMnYySDhWOHptMCAzaDJ2Mkg4di0yem0tMSAySDV2LTJoMnYyem0wLTNINVY4aDJ2MnptOSA3SDh2LTJoOHYyem0wLTRoLTJ2LTJoMnYyem0wLTNoLTJWOGgydjJ6bTMgM2gtMnYtMmgydjJ6bTAtM2gtMlY4aDJ2MnoiLz4KPC9zdmc+Cg==);
  --jp-icon-launch: url(data:image/svg+xml;base64,PHN2ZyB2aWV3Qm94PSIwIDAgMzIgMzIiIHdpZHRoPSIzMiIgeG1sbnM9Imh0dHA6Ly93d3cudzMub3JnLzIwMDAvc3ZnIj4KICA8ZyBjbGFzcz0ianAtaWNvbjMganAtaWNvbi1zZWxlY3RhYmxlIiBmaWxsPSIjNjE2MTYxIj4KICAgIDxwYXRoIGQ9Ik0yNiwyOEg2YTIuMDAyNywyLjAwMjcsMCwwLDEtMi0yVjZBMi4wMDI3LDIuMDAyNywwLDAsMSw2LDRIMTZWNkg2VjI2SDI2VjE2aDJWMjZBMi4wMDI3LDIuMDAyNywwLDAsMSwyNiwyOFoiLz4KICAgIDxwb2x5Z29uIHBvaW50cz0iMjAgMiAyMCA0IDI2LjU4NiA0IDE4IDEyLjU4NiAxOS40MTQgMTQgMjggNS40MTQgMjggMTIgMzAgMTIgMzAgMiAyMCAyIi8+CiAgPC9nPgo8L3N2Zz4K);
  --jp-icon-launcher: url(data:image/svg+xml;base64,PHN2ZyB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIHdpZHRoPSIxNiIgdmlld0JveD0iMCAwIDI0IDI0Ij4KICA8cGF0aCBjbGFzcz0ianAtaWNvbjMganAtaWNvbi1zZWxlY3RhYmxlIiBmaWxsPSIjNjE2MTYxIiBkPSJNMTkgMTlINVY1aDdWM0g1YTIgMiAwIDAwLTIgMnYxNGEyIDIgMCAwMDIgMmgxNGMxLjEgMCAyLS45IDItMnYtN2gtMnY3ek0xNCAzdjJoMy41OWwtOS44MyA5LjgzIDEuNDEgMS40MUwxOSA2LjQxVjEwaDJWM2gtN3oiLz4KPC9zdmc+Cg==);
  --jp-icon-line-form: url(data:image/svg+xml;base64,PHN2ZyB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIHdpZHRoPSIxNiIgdmlld0JveD0iMCAwIDI0IDI0Ij4KICAgIDxwYXRoIGZpbGw9IndoaXRlIiBkPSJNNS44OCA0LjEyTDEzLjc2IDEybC03Ljg4IDcuODhMOCAyMmwxMC0xMEw4IDJ6Ii8+Cjwvc3ZnPgo=);
  --jp-icon-link: url(data:image/svg+xml;base64,PHN2ZyB2aWV3Qm94PSIwIDAgMjQgMjQiIHdpZHRoPSIxNiIgeG1sbnM9Imh0dHA6Ly93d3cudzMub3JnLzIwMDAvc3ZnIj4KICA8ZyBjbGFzcz0ianAtaWNvbjMiIGZpbGw9IiM2MTYxNjEiPgogICAgPHBhdGggZD0iTTMuOSAxMmMwLTEuNzEgMS4zOS0zLjEgMy4xLTMuMWg0VjdIN2MtMi43NiAwLTUgMi4yNC01IDVzMi4yNCA1IDUgNWg0di0xLjlIN2MtMS43MSAwLTMuMS0xLjM5LTMuMS0zLjF6TTggMTNoOHYtMkg4djJ6bTktNmgtNHYxLjloNGMxLjcxIDAgMy4xIDEuMzkgMy4xIDMuMXMtMS4zOSAzLjEtMy4xIDMuMWgtNFYxN2g0YzIuNzYgMCA1LTIuMjQgNS01cy0yLjI0LTUtNS01eiIvPgogIDwvZz4KPC9zdmc+Cg==);
  --jp-icon-list: url(data:image/svg+xml;base64,PHN2ZyB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIHdpZHRoPSIxNiIgdmlld0JveD0iMCAwIDI0IDI0Ij4KICAgIDxwYXRoIGNsYXNzPSJqcC1pY29uMiBqcC1pY29uLXNlbGVjdGFibGUiIGZpbGw9IiM2MTYxNjEiIGQ9Ik0xOSA1djE0SDVWNWgxNG0xLjEtMkgzLjljLS41IDAtLjkuNC0uOS45djE2LjJjMCAuNC40LjkuOS45aDE2LjJjLjQgMCAuOS0uNS45LS45VjMuOWMwLS41LS41LS45LS45LS45ek0xMSA3aDZ2MmgtNlY3em0wIDRoNnYyaC02di0yem0wIDRoNnYyaC02ek03IDdoMnYySDd6bTAgNGgydjJIN3ptMCA0aDJ2Mkg3eiIvPgo8L3N2Zz4K);
  --jp-icon-markdown: url(data:image/svg+xml;base64,PHN2ZyB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIHdpZHRoPSIxNiIgdmlld0JveD0iMCAwIDIyIDIyIj4KICA8cGF0aCBjbGFzcz0ianAtaWNvbi1jb250cmFzdDAganAtaWNvbi1zZWxlY3RhYmxlIiBmaWxsPSIjN0IxRkEyIiBkPSJNNSAxNC45aDEybC02LjEgNnptOS40LTYuOGMwLTEuMy0uMS0yLjktLjEtNC41LS40IDEuNC0uOSAyLjktMS4zIDQuM2wtMS4zIDQuM2gtMkw4LjUgNy45Yy0uNC0xLjMtLjctMi45LTEtNC4zLS4xIDEuNi0uMSAzLjItLjIgNC42TDcgMTIuNEg0LjhsLjctMTFoMy4zTDEwIDVjLjQgMS4yLjcgMi43IDEgMy45LjMtMS4yLjctMi42IDEtMy45bDEuMi0zLjdoMy4zbC42IDExaC0yLjRsLS4zLTQuMnoiLz4KPC9zdmc+Cg==);
  --jp-icon-move-down: url(data:image/svg+xml;base64,PHN2ZyB3aWR0aD0iMTQiIGhlaWdodD0iMTQiIHZpZXdCb3g9IjAgMCAxNCAxNCIgZmlsbD0ibm9uZSIgeG1sbnM9Imh0dHA6Ly93d3cudzMub3JnLzIwMDAvc3ZnIj4KPHBhdGggY2xhc3M9ImpwLWljb24zIiBkPSJNMTIuNDcxIDcuNTI4OTlDMTIuNzYzMiA3LjIzNjg0IDEyLjc2MzIgNi43NjMxNiAxMi40NzEgNi40NzEwMVY2LjQ3MTAxQzEyLjE3OSA2LjE3OTA1IDExLjcwNTcgNi4xNzg4NCAxMS40MTM1IDYuNDcwNTRMNy43NSAxMC4xMjc1VjEuNzVDNy43NSAxLjMzNTc5IDcuNDE0MjEgMSA3IDFWMUM2LjU4NTc5IDEgNi4yNSAxLjMzNTc5IDYuMjUgMS43NVYxMC4xMjc1TDIuNTk3MjYgNi40NjgyMkMyLjMwMzM4IDYuMTczODEgMS44MjY0MSA2LjE3MzU5IDEuNTMyMjYgNi40Njc3NFY2LjQ2Nzc0QzEuMjM4MyA2Ljc2MTcgMS4yMzgzIDcuMjM4MyAxLjUzMjI2IDcuNTMyMjZMNi4yOTI4OSAxMi4yOTI5QzYuNjgzNDIgMTIuNjgzNCA3LjMxNjU4IDEyLjY4MzQgNy43MDcxMSAxMi4yOTI5TDEyLjQ3MSA3LjUyODk5WiIgZmlsbD0iIzYxNjE2MSIvPgo8L3N2Zz4K);
  --jp-icon-move-up: url(data:image/svg+xml;base64,PHN2ZyB3aWR0aD0iMTQiIGhlaWdodD0iMTQiIHZpZXdCb3g9IjAgMCAxNCAxNCIgZmlsbD0ibm9uZSIgeG1sbnM9Imh0dHA6Ly93d3cudzMub3JnLzIwMDAvc3ZnIj4KPHBhdGggY2xhc3M9ImpwLWljb24zIiBkPSJNMS41Mjg5OSA2LjQ3MTAxQzEuMjM2ODQgNi43NjMxNiAxLjIzNjg0IDcuMjM2ODQgMS41Mjg5OSA3LjUyODk5VjcuNTI4OTlDMS44MjA5NSA3LjgyMDk1IDIuMjk0MjYgNy44MjExNiAyLjU4NjQ5IDcuNTI5NDZMNi4yNSAzLjg3MjVWMTIuMjVDNi4yNSAxMi42NjQyIDYuNTg1NzkgMTMgNyAxM1YxM0M3LjQxNDIxIDEzIDcuNzUgMTIuNjY0MiA3Ljc1IDEyLjI1VjMuODcyNUwxMS40MDI3IDcuNTMxNzhDMTEuNjk2NiA3LjgyNjE5IDEyLjE3MzYgNy44MjY0MSAxMi40Njc3IDcuNTMyMjZWNy41MzIyNkMxMi43NjE3IDcuMjM4MyAxMi43NjE3IDYuNzYxNyAxMi40Njc3IDYuNDY3NzRMNy43MDcxMSAxLjcwNzExQzcuMzE2NTggMS4zMTY1OCA2LjY4MzQyIDEuMzE2NTggNi4yOTI4OSAxLjcwNzExTDEuNTI4OTkgNi40NzEwMVoiIGZpbGw9IiM2MTYxNjEiLz4KPC9zdmc+Cg==);
  --jp-icon-new-folder: url(data:image/svg+xml;base64,PHN2ZyB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIHdpZHRoPSIxNiIgdmlld0JveD0iMCAwIDI0IDI0Ij4KICA8ZyBjbGFzcz0ianAtaWNvbjMiIGZpbGw9IiM2MTYxNjEiPgogICAgPHBhdGggZD0iTTIwIDZoLThsLTItMkg0Yy0xLjExIDAtMS45OS44OS0xLjk5IDJMMiAxOGMwIDEuMTEuODkgMiAyIDJoMTZjMS4xMSAwIDItLjg5IDItMlY4YzAtMS4xMS0uODktMi0yLTJ6bS0xIDhoLTN2M2gtMnYtM2gtM3YtMmgzVjloMnYzaDN2MnoiLz4KICA8L2c+Cjwvc3ZnPgo=);
  --jp-icon-not-trusted: url(data:image/svg+xml;base64,PHN2ZyBmaWxsPSJub25lIiB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIHdpZHRoPSIxNiIgdmlld0JveD0iMCAwIDI1IDI1Ij4KICAgIDxwYXRoIGNsYXNzPSJqcC1pY29uMiIgc3Ryb2tlPSIjMzMzMzMzIiBzdHJva2Utd2lkdGg9IjIiIHRyYW5zZm9ybT0idHJhbnNsYXRlKDMgMykiIGQ9Ik0xLjg2MDk0IDExLjQ0MDlDMC44MjY0NDggOC43NzAyNyAwLjg2Mzc3OSA2LjA1NzY0IDEuMjQ5MDcgNC4xOTkzMkMyLjQ4MjA2IDMuOTMzNDcgNC4wODA2OCAzLjQwMzQ3IDUuNjAxMDIgMi44NDQ5QzcuMjM1NDkgMi4yNDQ0IDguODU2NjYgMS41ODE1IDkuOTg3NiAxLjA5NTM5QzExLjA1OTcgMS41ODM0MSAxMi42MDk0IDIuMjQ0NCAxNC4yMTggMi44NDMzOUMxNS43NTAzIDMuNDEzOTQgMTcuMzk5NSAzLjk1MjU4IDE4Ljc1MzkgNC4yMTM4NUMxOS4xMzY0IDYuMDcxNzcgMTkuMTcwOSA4Ljc3NzIyIDE4LjEzOSAxMS40NDA5QzE3LjAzMDMgMTQuMzAzMiAxNC42NjY4IDE3LjE4NDQgOS45OTk5OSAxOC45MzU0QzUuMzMzMTkgMTcuMTg0NCAyLjk2OTY4IDE0LjMwMzIgMS44NjA5NCAxMS40NDA5WiIvPgogICAgPHBhdGggY2xhc3M9ImpwLWljb24yIiBzdHJva2U9IiMzMzMzMzMiIHN0cm9rZS13aWR0aD0iMiIgdHJhbnNmb3JtPSJ0cmFuc2xhdGUoOS4zMTU5MiA5LjMyMDMxKSIgZD0iTTcuMzY4NDIgMEwwIDcuMzY0NzkiLz4KICAgIDxwYXRoIGNsYXNzPSJqcC1pY29uMiIgc3Ryb2tlPSIjMzMzMzMzIiBzdHJva2Utd2lkdGg9IjIiIHRyYW5zZm9ybT0idHJhbnNsYXRlKDkuMzE1OTIgMTYuNjgzNikgc2NhbGUoMSAtMSkiIGQ9Ik03LjM2ODQyIDBMMCA3LjM2NDc5Ii8+Cjwvc3ZnPgo=);
  --jp-icon-notebook: url(data:image/svg+xml;base64,PHN2ZyB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIHdpZHRoPSIxNiIgdmlld0JveD0iMCAwIDIyIDIyIj4KICA8ZyBjbGFzcz0ianAtbm90ZWJvb2staWNvbi1jb2xvciBqcC1pY29uLXNlbGVjdGFibGUiIGZpbGw9IiNFRjZDMDAiPgogICAgPHBhdGggZD0iTTE4LjcgMy4zdjE1LjRIMy4zVjMuM2gxNS40bTEuNS0xLjVIMS44djE4LjNoMTguM2wuMS0xOC4zeiIvPgogICAgPHBhdGggZD0iTTE2LjUgMTYuNWwtNS40LTQuMy01LjYgNC4zdi0xMWgxMXoiLz4KICA8L2c+Cjwvc3ZnPgo=);
  --jp-icon-numbering: url(data:image/svg+xml;base64,PHN2ZyB3aWR0aD0iMjIiIGhlaWdodD0iMjIiIHZpZXdCb3g9IjAgMCAyOCAyOCIgeG1sbnM9Imh0dHA6Ly93d3cudzMub3JnLzIwMDAvc3ZnIj4KCTxnIGNsYXNzPSJqcC1pY29uMyIgZmlsbD0iIzYxNjE2MSI+CgkJPHBhdGggZD0iTTQgMTlINlYxOS41SDVWMjAuNUg2VjIxSDRWMjJIN1YxOEg0VjE5Wk01IDEwSDZWNkg0VjdINVYxMFpNNCAxM0g1LjhMNCAxNS4xVjE2SDdWMTVINS4yTDcgMTIuOVYxMkg0VjEzWk05IDdWOUgyM1Y3SDlaTTkgMjFIMjNWMTlIOVYyMVpNOSAxNUgyM1YxM0g5VjE1WiIvPgoJPC9nPgo8L3N2Zz4K);
  --jp-icon-offline-bolt: url(data:image/svg+xml;base64,PHN2ZyB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIHZpZXdCb3g9IjAgMCAyNCAyNCIgd2lkdGg9IjE2Ij4KICA8ZyBjbGFzcz0ianAtaWNvbjMiIGZpbGw9IiM2MTYxNjEiPgogICAgPHBhdGggZD0iTTEyIDIuMDJjLTUuNTEgMC05Ljk4IDQuNDctOS45OCA5Ljk4czQuNDcgOS45OCA5Ljk4IDkuOTggOS45OC00LjQ3IDkuOTgtOS45OFMxNy41MSAyLjAyIDEyIDIuMDJ6TTExLjQ4IDIwdi02LjI2SDhMMTMgNHY2LjI2aDMuMzVMMTEuNDggMjB6Ii8+CiAgPC9nPgo8L3N2Zz4K);
  --jp-icon-palette: url(data:image/svg+xml;base64,PHN2ZyB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIHdpZHRoPSIxNiIgdmlld0JveD0iMCAwIDI0IDI0Ij4KICA8ZyBjbGFzcz0ianAtaWNvbjMiIGZpbGw9IiM2MTYxNjEiPgogICAgPHBhdGggZD0iTTE4IDEzVjIwSDRWNkg5LjAyQzkuMDcgNS4yOSA5LjI0IDQuNjIgOS41IDRINEMyLjkgNCAyIDQuOSAyIDZWMjBDMiAyMS4xIDIuOSAyMiA0IDIySDE4QzE5LjEgMjIgMjAgMjEuMSAyMCAyMFYxNUwxOCAxM1pNMTkuMyA4Ljg5QzE5Ljc0IDguMTkgMjAgNy4zOCAyMCA2LjVDMjAgNC4wMSAxNy45OSAyIDE1LjUgMkMxMy4wMSAyIDExIDQuMDEgMTEgNi41QzExIDguOTkgMTMuMDEgMTEgMTUuNDkgMTFDMTYuMzcgMTEgMTcuMTkgMTAuNzQgMTcuODggMTAuM0wyMSAxMy40MkwyMi40MiAxMkwxOS4zIDguODlaTTE1LjUgOUMxNC4xMiA5IDEzIDcuODggMTMgNi41QzEzIDUuMTIgMTQuMTIgNCAxNS41IDRDMTYuODggNCAxOCA1LjEyIDE4IDYuNUMxOCA3Ljg4IDE2Ljg4IDkgMTUuNSA5WiIvPgogICAgPHBhdGggZmlsbC1ydWxlPSJldmVub2RkIiBjbGlwLXJ1bGU9ImV2ZW5vZGQiIGQ9Ik00IDZIOS4wMTg5NEM5LjAwNjM5IDYuMTY1MDIgOSA2LjMzMTc2IDkgNi41QzkgOC44MTU3NyAxMC4yMTEgMTAuODQ4NyAxMi4wMzQzIDEySDlWMTRIMTZWMTIuOTgxMUMxNi41NzAzIDEyLjkzNzcgMTcuMTIgMTIuODIwNyAxNy42Mzk2IDEyLjYzOTZMMTggMTNWMjBINFY2Wk04IDhINlYxMEg4VjhaTTYgMTJIOFYxNEg2VjEyWk04IDE2SDZWMThIOFYxNlpNOSAxNkgxNlYxOEg5VjE2WiIvPgogIDwvZz4KPC9zdmc+Cg==);
  --jp-icon-paste: url(data:image/svg+xml;base64,PHN2ZyBoZWlnaHQ9IjI0IiB2aWV3Qm94PSIwIDAgMjQgMjQiIHdpZHRoPSIyNCIgeG1sbnM9Imh0dHA6Ly93d3cudzMub3JnLzIwMDAvc3ZnIj4KICAgIDxnIGNsYXNzPSJqcC1pY29uMyIgZmlsbD0iIzYxNjE2MSI+CiAgICAgICAgPHBhdGggZD0iTTE5IDJoLTQuMThDMTQuNC44NCAxMy4zIDAgMTIgMGMtMS4zIDAtMi40Ljg0LTIuODIgMkg1Yy0xLjEgMC0yIC45LTIgMnYxNmMwIDEuMS45IDIgMiAyaDE0YzEuMSAwIDItLjkgMi0yVjRjMC0xLjEtLjktMi0yLTJ6bS03IDBjLjU1IDAgMSAuNDUgMSAxcy0uNDUgMS0xIDEtMS0uNDUtMS0xIC40NS0xIDEtMXptNyAxOEg1VjRoMnYzaDEwVjRoMnYxNnoiLz4KICAgIDwvZz4KPC9zdmc+Cg==);
  --jp-icon-pdf: url(data:image/svg+xml;base64,PHN2ZwogICB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIHZpZXdCb3g9IjAgMCAyMiAyMiIgd2lkdGg9IjE2Ij4KICAgIDxwYXRoIHRyYW5zZm9ybT0icm90YXRlKDQ1KSIgY2xhc3M9ImpwLWljb24tc2VsZWN0YWJsZSIgZmlsbD0iI0ZGMkEyQSIKICAgICAgIGQ9Im0gMjIuMzQ0MzY5LC0zLjAxNjM2NDIgaCA1LjYzODYwNCB2IDEuNTc5MjQzMyBoIC0zLjU0OTIyNyB2IDEuNTA4NjkyOTkgaCAzLjMzNzU3NiBWIDEuNjUwODE1NCBoIC0zLjMzNzU3NiB2IDMuNDM1MjYxMyBoIC0yLjA4OTM3NyB6IG0gLTcuMTM2NDQ0LDEuNTc5MjQzMyB2IDQuOTQzOTU0MyBoIDAuNzQ4OTIgcSAxLjI4MDc2MSwwIDEuOTUzNzAzLC0wLjYzNDk1MzUgMC42NzgzNjksLTAuNjM0OTUzNSAwLjY3ODM2OSwtMS44NDUxNjQxIDAsLTEuMjA0NzgzNTUgLTAuNjcyOTQyLC0xLjgzNDMxMDExIC0wLjY3Mjk0MiwtMC42Mjk1MjY1OSAtMS45NTkxMywtMC42Mjk1MjY1OSB6IG0gLTIuMDg5Mzc3LC0xLjU3OTI0MzMgaCAyLjIwMzM0MyBxIDEuODQ1MTY0LDAgMi43NDYwMzksMC4yNjU5MjA3IDAuOTA2MzAxLDAuMjYwNDkzNyAxLjU1MjEwOCwwLjg5MDAyMDMgMC41Njk4MywwLjU0ODEyMjMgMC44NDY2MDUsMS4yNjQ0ODAwNiAwLjI3Njc3NCwwLjcxNjM1NzgxIDAuMjc2Nzc0LDEuNjIyNjU4OTQgMCwwLjkxNzE1NTEgLTAuMjc2Nzc0LDEuNjM4OTM5OSAtMC4yNzY3NzUsMC43MTYzNTc4IC0wLjg0NjYwNSwxLjI2NDQ4IC0wLjY1MTIzNCwwLjYyOTUyNjYgLTEuNTYyOTYyLDAuODk1NDQ3MyAtMC45MTE3MjgsMC4yNjA0OTM3IC0yLjczNTE4NSwwLjI2MDQ5MzcgaCAtMi4yMDMzNDMgeiBtIC04LjE0NTg1NjUsMCBoIDMuNDY3ODIzIHEgMS41NDY2ODE2LDAgMi4zNzE1Nzg1LDAuNjg5MjIzIDAuODMwMzI0LDAuNjgzNzk2MSAwLjgzMDMyNCwxLjk1MzcwMzE0IDAsMS4yNzUzMzM5NyAtMC44MzAzMjQsMS45NjQ1NTcwNiBRIDkuOTg3MTk2MSwyLjI3NDkxNSA4LjQ0MDUxNDUsMi4yNzQ5MTUgSCA3LjA2MjA2ODQgViA1LjA4NjA3NjcgSCA0Ljk3MjY5MTUgWiBtIDIuMDg5Mzc2OSwxLjUxNDExOTkgdiAyLjI2MzAzOTQzIGggMS4xNTU5NDEgcSAwLjYwNzgxODgsMCAwLjkzODg2MjksLTAuMjkzMDU1NDcgMC4zMzEwNDQxLC0wLjI5ODQ4MjQxIDAuMzMxMDQ0MSwtMC44NDExNzc3MiAwLC0wLjU0MjY5NTMxIC0wLjMzMTA0NDEsLTAuODM1NzUwNzQgLTAuMzMxMDQ0MSwtMC4yOTMwNTU1IC0wLjkzODg2MjksLTAuMjkzMDU1NSB6IgovPgo8L3N2Zz4K);
  --jp-icon-python: url(data:image/svg+xml;base64,PHN2ZyB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIHdpZHRoPSIxNiIgdmlld0JveD0iLTEwIC0xMCAxMzEuMTYxMzYxNjk0MzM1OTQgMTMyLjM4ODk5OTkzODk2NDg0Ij4KICA8cGF0aCBjbGFzcz0ianAtaWNvbi1zZWxlY3RhYmxlIiBmaWxsPSIjMzA2OTk4IiBkPSJNIDU0LjkxODc4NSw5LjE5Mjc0MjFlLTQgQyA1MC4zMzUxMzIsMC4wMjIyMTcyNyA0NS45NTc4NDYsMC40MTMxMzY5NyA0Mi4xMDYyODUsMS4wOTQ2NjkzIDMwLjc2MDA2OSwzLjA5OTE3MzEgMjguNzAwMDM2LDcuMjk0NzcxNCAyOC43MDAwMzUsMTUuMDMyMTY5IHYgMTAuMjE4NzUgaCAyNi44MTI1IHYgMy40MDYyNSBoIC0yNi44MTI1IC0xMC4wNjI1IGMgLTcuNzkyNDU5LDAgLTE0LjYxNTc1ODgsNC42ODM3MTcgLTE2Ljc0OTk5OTgsMTMuNTkzNzUgLTIuNDYxODE5OTgsMTAuMjEyOTY2IC0yLjU3MTAxNTA4LDE2LjU4NjAyMyAwLDI3LjI1IDEuOTA1OTI4Myw3LjkzNzg1MiA2LjQ1NzU0MzIsMTMuNTkzNzQ4IDE0LjI0OTk5OTgsMTMuNTkzNzUgaCA5LjIxODc1IHYgLTEyLjI1IGMgMCwtOC44NDk5MDIgNy42NTcxNDQsLTE2LjY1NjI0OCAxNi43NSwtMTYuNjU2MjUgaCAyNi43ODEyNSBjIDcuNDU0OTUxLDAgMTMuNDA2MjUzLC02LjEzODE2NCAxMy40MDYyNSwtMTMuNjI1IHYgLTI1LjUzMTI1IGMgMCwtNy4yNjYzMzg2IC02LjEyOTk4LC0xMi43MjQ3NzcxIC0xMy40MDYyNSwtMTMuOTM3NDk5NyBDIDY0LjI4MTU0OCwwLjMyNzk0Mzk3IDU5LjUwMjQzOCwtMC4wMjAzNzkwMyA1NC45MTg3ODUsOS4xOTI3NDIxZS00IFogbSAtMTQuNSw4LjIxODc1MDEyNTc5IGMgMi43Njk1NDcsMCA1LjAzMTI1LDIuMjk4NjQ1NiA1LjAzMTI1LDUuMTI0OTk5NiAtMmUtNiwyLjgxNjMzNiAtMi4yNjE3MDMsNS4wOTM3NSAtNS4wMzEyNSw1LjA5Mzc1IC0yLjc3OTQ3NiwtMWUtNiAtNS4wMzEyNSwtMi4yNzc0MTUgLTUuMDMxMjUsLTUuMDkzNzUgLTEwZS03LC0yLjgyNjM1MyAyLjI1MTc3NCwtNS4xMjQ5OTk2IDUuMDMxMjUsLTUuMTI0OTk5NiB6Ii8+CiAgPHBhdGggY2xhc3M9ImpwLWljb24tc2VsZWN0YWJsZSIgZmlsbD0iI2ZmZDQzYiIgZD0ibSA4NS42Mzc1MzUsMjguNjU3MTY5IHYgMTEuOTA2MjUgYyAwLDkuMjMwNzU1IC03LjgyNTg5NSwxNi45OTk5OTkgLTE2Ljc1LDE3IGggLTI2Ljc4MTI1IGMgLTcuMzM1ODMzLDAgLTEzLjQwNjI0OSw2LjI3ODQ4MyAtMTMuNDA2MjUsMTMuNjI1IHYgMjUuNTMxMjQ3IGMgMCw3LjI2NjM0NCA2LjMxODU4OCwxMS41NDAzMjQgMTMuNDA2MjUsMTMuNjI1MDA0IDguNDg3MzMxLDIuNDk1NjEgMTYuNjI2MjM3LDIuOTQ2NjMgMjYuNzgxMjUsMCA2Ljc1MDE1NSwtMS45NTQzOSAxMy40MDYyNTMsLTUuODg3NjEgMTMuNDA2MjUsLTEzLjYyNTAwNCBWIDg2LjUwMDkxOSBoIC0yNi43ODEyNSB2IC0zLjQwNjI1IGggMjYuNzgxMjUgMTMuNDA2MjU0IGMgNy43OTI0NjEsMCAxMC42OTYyNTEsLTUuNDM1NDA4IDEzLjQwNjI0MSwtMTMuNTkzNzUgMi43OTkzMywtOC4zOTg4ODYgMi42ODAyMiwtMTYuNDc1Nzc2IDAsLTI3LjI1IC0xLjkyNTc4LC03Ljc1NzQ0MSAtNS42MDM4NywtMTMuNTkzNzUgLTEzLjQwNjI0MSwtMTMuNTkzNzUgeiBtIC0xNS4wNjI1LDY0LjY1NjI1IGMgMi43Nzk0NzgsM2UtNiA1LjAzMTI1LDIuMjc3NDE3IDUuMDMxMjUsNS4wOTM3NDcgLTJlLTYsMi44MjYzNTQgLTIuMjUxNzc1LDUuMTI1MDA0IC01LjAzMTI1LDUuMTI1MDA0IC0yLjc2OTU1LDAgLTUuMDMxMjUsLTIuMjk4NjUgLTUuMDMxMjUsLTUuMTI1MDA0IDJlLTYsLTIuODE2MzMgMi4yNjE2OTcsLTUuMDkzNzQ3IDUuMDMxMjUsLTUuMDkzNzQ3IHoiLz4KPC9zdmc+Cg==);
  --jp-icon-r-kernel: url(data:image/svg+xml;base64,PHN2ZyB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIHdpZHRoPSIxNiIgdmlld0JveD0iMCAwIDIyIDIyIj4KICA8cGF0aCBjbGFzcz0ianAtaWNvbi1jb250cmFzdDMganAtaWNvbi1zZWxlY3RhYmxlIiBmaWxsPSIjMjE5NkYzIiBkPSJNNC40IDIuNWMxLjItLjEgMi45LS4zIDQuOS0uMyAyLjUgMCA0LjEuNCA1LjIgMS4zIDEgLjcgMS41IDEuOSAxLjUgMy41IDAgMi0xLjQgMy41LTIuOSA0LjEgMS4yLjQgMS43IDEuNiAyLjIgMyAuNiAxLjkgMSAzLjkgMS4zIDQuNmgtMy44Yy0uMy0uNC0uOC0xLjctMS4yLTMuN3MtMS4yLTIuNi0yLjYtMi42aC0uOXY2LjRINC40VjIuNXptMy43IDYuOWgxLjRjMS45IDAgMi45LS45IDIuOS0yLjNzLTEtMi4zLTIuOC0yLjNjLS43IDAtMS4zIDAtMS42LjJ2NC41aC4xdi0uMXoiLz4KPC9zdmc+Cg==);
  --jp-icon-react: url(data:image/svg+xml;base64,PHN2ZyB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIHdpZHRoPSIxNiIgdmlld0JveD0iMTUwIDE1MCA1NDEuOSAyOTUuMyI+CiAgPGcgY2xhc3M9ImpwLWljb24tYnJhbmQyIGpwLWljb24tc2VsZWN0YWJsZSIgZmlsbD0iIzYxREFGQiI+CiAgICA8cGF0aCBkPSJNNjY2LjMgMjk2LjVjMC0zMi41LTQwLjctNjMuMy0xMDMuMS04Mi40IDE0LjQtNjMuNiA4LTExNC4yLTIwLjItMTMwLjQtNi41LTMuOC0xNC4xLTUuNi0yMi40LTUuNnYyMi4zYzQuNiAwIDguMy45IDExLjQgMi42IDEzLjYgNy44IDE5LjUgMzcuNSAxNC45IDc1LjctMS4xIDkuNC0yLjkgMTkuMy01LjEgMjkuNC0xOS42LTQuOC00MS04LjUtNjMuNS0xMC45LTEzLjUtMTguNS0yNy41LTM1LjMtNDEuNi01MCAzMi42LTMwLjMgNjMuMi00Ni45IDg0LTQ2LjlWNzhjLTI3LjUgMC02My41IDE5LjYtOTkuOSA1My42LTM2LjQtMzMuOC03Mi40LTUzLjItOTkuOS01My4ydjIyLjNjMjAuNyAwIDUxLjQgMTYuNSA4NCA0Ni42LTE0IDE0LjctMjggMzEuNC00MS4zIDQ5LjktMjIuNiAyLjQtNDQgNi4xLTYzLjYgMTEtMi4zLTEwLTQtMTkuNy01LjItMjktNC43LTM4LjIgMS4xLTY3LjkgMTQuNi03NS44IDMtMS44IDYuOS0yLjYgMTEuNS0yLjZWNzguNWMtOC40IDAtMTYgMS44LTIyLjYgNS42LTI4LjEgMTYuMi0zNC40IDY2LjctMTkuOSAxMzAuMS02Mi4yIDE5LjItMTAyLjcgNDkuOS0xMDIuNyA4Mi4zIDAgMzIuNSA0MC43IDYzLjMgMTAzLjEgODIuNC0xNC40IDYzLjYtOCAxMTQuMiAyMC4yIDEzMC40IDYuNSAzLjggMTQuMSA1LjYgMjIuNSA1LjYgMjcuNSAwIDYzLjUtMTkuNiA5OS45LTUzLjYgMzYuNCAzMy44IDcyLjQgNTMuMiA5OS45IDUzLjIgOC40IDAgMTYtMS44IDIyLjYtNS42IDI4LjEtMTYuMiAzNC40LTY2LjcgMTkuOS0xMzAuMSA2Mi0xOS4xIDEwMi41LTQ5LjkgMTAyLjUtODIuM3ptLTEzMC4yLTY2LjdjLTMuNyAxMi45LTguMyAyNi4yLTEzLjUgMzkuNS00LjEtOC04LjQtMTYtMTMuMS0yNC00LjYtOC05LjUtMTUuOC0xNC40LTIzLjQgMTQuMiAyLjEgMjcuOSA0LjcgNDEgNy45em0tNDUuOCAxMDYuNWMtNy44IDEzLjUtMTUuOCAyNi4zLTI0LjEgMzguMi0xNC45IDEuMy0zMCAyLTQ1LjIgMi0xNS4xIDAtMzAuMi0uNy00NS0xLjktOC4zLTExLjktMTYuNC0yNC42LTI0LjItMzgtNy42LTEzLjEtMTQuNS0yNi40LTIwLjgtMzkuOCA2LjItMTMuNCAxMy4yLTI2LjggMjAuNy0zOS45IDcuOC0xMy41IDE1LjgtMjYuMyAyNC4xLTM4LjIgMTQuOS0xLjMgMzAtMiA0NS4yLTIgMTUuMSAwIDMwLjIuNyA0NSAxLjkgOC4zIDExLjkgMTYuNCAyNC42IDI0LjIgMzggNy42IDEzLjEgMTQuNSAyNi40IDIwLjggMzkuOC02LjMgMTMuNC0xMy4yIDI2LjgtMjAuNyAzOS45em0zMi4zLTEzYzUuNCAxMy40IDEwIDI2LjggMTMuOCAzOS44LTEzLjEgMy4yLTI2LjkgNS45LTQxLjIgOCA0LjktNy43IDkuOC0xNS42IDE0LjQtMjMuNyA0LjYtOCA4LjktMTYuMSAxMy0yNC4xek00MjEuMiA0MzBjLTkuMy05LjYtMTguNi0yMC4zLTI3LjgtMzIgOSAuNCAxOC4yLjcgMjcuNS43IDkuNCAwIDE4LjctLjIgMjcuOC0uNy05IDExLjctMTguMyAyMi40LTI3LjUgMzJ6bS03NC40LTU4LjljLTE0LjItMi4xLTI3LjktNC43LTQxLTcuOSAzLjctMTIuOSA4LjMtMjYuMiAxMy41LTM5LjUgNC4xIDggOC40IDE2IDEzLjEgMjQgNC43IDggOS41IDE1LjggMTQuNCAyMy40ek00MjAuNyAxNjNjOS4zIDkuNiAxOC42IDIwLjMgMjcuOCAzMi05LS40LTE4LjItLjctMjcuNS0uNy05LjQgMC0xOC43LjItMjcuOC43IDktMTEuNyAxOC4zLTIyLjQgMjcuNS0zMnptLTc0IDU4LjljLTQuOSA3LjctOS44IDE1LjYtMTQuNCAyMy43LTQuNiA4LTguOSAxNi0xMyAyNC01LjQtMTMuNC0xMC0yNi44LTEzLjgtMzkuOCAxMy4xLTMuMSAyNi45LTUuOCA0MS4yLTcuOXptLTkwLjUgMTI1LjJjLTM1LjQtMTUuMS01OC4zLTM0LjktNTguMy01MC42IDAtMTUuNyAyMi45LTM1LjYgNTguMy01MC42IDguNi0zLjcgMTgtNyAyNy43LTEwLjEgNS43IDE5LjYgMTMuMiA0MCAyMi41IDYwLjktOS4yIDIwLjgtMTYuNiA0MS4xLTIyLjIgNjAuNi05LjktMy4xLTE5LjMtNi41LTI4LTEwLjJ6TTMxMCA0OTBjLTEzLjYtNy44LTE5LjUtMzcuNS0xNC45LTc1LjcgMS4xLTkuNCAyLjktMTkuMyA1LjEtMjkuNCAxOS42IDQuOCA0MSA4LjUgNjMuNSAxMC45IDEzLjUgMTguNSAyNy41IDM1LjMgNDEuNiA1MC0zMi42IDMwLjMtNjMuMiA0Ni45LTg0IDQ2LjktNC41LS4xLTguMy0xLTExLjMtMi43em0yMzcuMi03Ni4yYzQuNyAzOC4yLTEuMSA2Ny45LTE0LjYgNzUuOC0zIDEuOC02LjkgMi42LTExLjUgMi42LTIwLjcgMC01MS40LTE2LjUtODQtNDYuNiAxNC0xNC43IDI4LTMxLjQgNDEuMy00OS45IDIyLjYtMi40IDQ0LTYuMSA2My42LTExIDIuMyAxMC4xIDQuMSAxOS44IDUuMiAyOS4xem0zOC41LTY2LjdjLTguNiAzLjctMTggNy0yNy43IDEwLjEtNS43LTE5LjYtMTMuMi00MC0yMi41LTYwLjkgOS4yLTIwLjggMTYuNi00MS4xIDIyLjItNjAuNiA5LjkgMy4xIDE5LjMgNi41IDI4LjEgMTAuMiAzNS40IDE1LjEgNTguMyAzNC45IDU4LjMgNTAuNi0uMSAxNS43LTIzIDM1LjYtNTguNCA1MC42ek0zMjAuOCA3OC40eiIvPgogICAgPGNpcmNsZSBjeD0iNDIwLjkiIGN5PSIyOTYuNSIgcj0iNDUuNyIvPgogIDwvZz4KPC9zdmc+Cg==);
  --jp-icon-redo: url(data:image/svg+xml;base64,PHN2ZyB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIGhlaWdodD0iMjQiIHZpZXdCb3g9IjAgMCAyNCAyNCIgd2lkdGg9IjE2Ij4KICA8ZyBjbGFzcz0ianAtaWNvbjMiIGZpbGw9IiM2MTYxNjEiPgogICAgICA8cGF0aCBkPSJNMCAwaDI0djI0SDB6IiBmaWxsPSJub25lIi8+PHBhdGggZD0iTTE4LjQgMTAuNkMxNi41NSA4Ljk5IDE0LjE1IDggMTEuNSA4Yy00LjY1IDAtOC41OCAzLjAzLTkuOTYgNy4yMkwzLjkgMTZjMS4wNS0zLjE5IDQuMDUtNS41IDcuNi01LjUgMS45NSAwIDMuNzMuNzIgNS4xMiAxLjg4TDEzIDE2aDlWN2wtMy42IDMuNnoiLz4KICA8L2c+Cjwvc3ZnPgo=);
  --jp-icon-refresh: url(data:image/svg+xml;base64,PHN2ZyB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIHdpZHRoPSIxNiIgdmlld0JveD0iMCAwIDE4IDE4Ij4KICAgIDxnIGNsYXNzPSJqcC1pY29uMyIgZmlsbD0iIzYxNjE2MSI+CiAgICAgICAgPHBhdGggZD0iTTkgMTMuNWMtMi40OSAwLTQuNS0yLjAxLTQuNS00LjVTNi41MSA0LjUgOSA0LjVjMS4yNCAwIDIuMzYuNTIgMy4xNyAxLjMzTDEwIDhoNVYzbC0xLjc2IDEuNzZDMTIuMTUgMy42OCAxMC42NiAzIDkgMyA1LjY5IDMgMy4wMSA1LjY5IDMuMDEgOVM1LjY5IDE1IDkgMTVjMi45NyAwIDUuNDMtMi4xNiA1LjktNWgtMS41MmMtLjQ2IDItMi4yNCAzLjUtNC4zOCAzLjV6Ii8+CiAgICA8L2c+Cjwvc3ZnPgo=);
  --jp-icon-regex: url(data:image/svg+xml;base64,PHN2ZyB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIHdpZHRoPSIxNiIgdmlld0JveD0iMCAwIDIwIDIwIj4KICA8ZyBjbGFzcz0ianAtaWNvbjIiIGZpbGw9IiM0MTQxNDEiPgogICAgPHJlY3QgeD0iMiIgeT0iMiIgd2lkdGg9IjE2IiBoZWlnaHQ9IjE2Ii8+CiAgPC9nPgoKICA8ZyBjbGFzcz0ianAtaWNvbi1hY2NlbnQyIiBmaWxsPSIjRkZGIj4KICAgIDxjaXJjbGUgY2xhc3M9InN0MiIgY3g9IjUuNSIgY3k9IjE0LjUiIHI9IjEuNSIvPgogICAgPHJlY3QgeD0iMTIiIHk9IjQiIGNsYXNzPSJzdDIiIHdpZHRoPSIxIiBoZWlnaHQ9IjgiLz4KICAgIDxyZWN0IHg9IjguNSIgeT0iNy41IiB0cmFuc2Zvcm09Im1hdHJpeCgwLjg2NiAtMC41IDAuNSAwLjg2NiAtMi4zMjU1IDcuMzIxOSkiIGNsYXNzPSJzdDIiIHdpZHRoPSI4IiBoZWlnaHQ9IjEiLz4KICAgIDxyZWN0IHg9IjEyIiB5PSI0IiB0cmFuc2Zvcm09Im1hdHJpeCgwLjUgLTAuODY2IDAuODY2IDAuNSAtMC42Nzc5IDE0LjgyNTIpIiBjbGFzcz0ic3QyIiB3aWR0aD0iMSIgaGVpZ2h0PSI4Ii8+CiAgPC9nPgo8L3N2Zz4K);
  --jp-icon-run: url(data:image/svg+xml;base64,PHN2ZyBoZWlnaHQ9IjI0IiB2aWV3Qm94PSIwIDAgMjQgMjQiIHdpZHRoPSIyNCIgeG1sbnM9Imh0dHA6Ly93d3cudzMub3JnLzIwMDAvc3ZnIj4KICAgIDxnIGNsYXNzPSJqcC1pY29uMyIgZmlsbD0iIzYxNjE2MSI+CiAgICAgICAgPHBhdGggZD0iTTggNXYxNGwxMS03eiIvPgogICAgPC9nPgo8L3N2Zz4K);
  --jp-icon-running: url(data:image/svg+xml;base64,PHN2ZyB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIHdpZHRoPSIxNiIgdmlld0JveD0iMCAwIDUxMiA1MTIiPgogIDxnIGNsYXNzPSJqcC1pY29uMyIgZmlsbD0iIzYxNjE2MSI+CiAgICA8cGF0aCBkPSJNMjU2IDhDMTE5IDggOCAxMTkgOCAyNTZzMTExIDI0OCAyNDggMjQ4IDI0OC0xMTEgMjQ4LTI0OFMzOTMgOCAyNTYgOHptOTYgMzI4YzAgOC44LTcuMiAxNi0xNiAxNkgxNzZjLTguOCAwLTE2LTcuMi0xNi0xNlYxNzZjMC04LjggNy4yLTE2IDE2LTE2aDE2MGM4LjggMCAxNiA3LjIgMTYgMTZ2MTYweiIvPgogIDwvZz4KPC9zdmc+Cg==);
  --jp-icon-save: url(data:image/svg+xml;base64,PHN2ZyBoZWlnaHQ9IjI0IiB2aWV3Qm94PSIwIDAgMjQgMjQiIHdpZHRoPSIyNCIgeG1sbnM9Imh0dHA6Ly93d3cudzMub3JnLzIwMDAvc3ZnIj4KICAgIDxnIGNsYXNzPSJqcC1pY29uMyIgZmlsbD0iIzYxNjE2MSI+CiAgICAgICAgPHBhdGggZD0iTTE3IDNINWMtMS4xMSAwLTIgLjktMiAydjE0YzAgMS4xLjg5IDIgMiAyaDE0YzEuMSAwIDItLjkgMi0yVjdsLTQtNHptLTUgMTZjLTEuNjYgMC0zLTEuMzQtMy0zczEuMzQtMyAzLTMgMyAxLjM0IDMgMy0xLjM0IDMtMyAzem0zLTEwSDVWNWgxMHY0eiIvPgogICAgPC9nPgo8L3N2Zz4K);
  --jp-icon-search: url(data:image/svg+xml;base64,PHN2ZyB2aWV3Qm94PSIwIDAgMTggMTgiIHdpZHRoPSIxNiIgeG1sbnM9Imh0dHA6Ly93d3cudzMub3JnLzIwMDAvc3ZnIj4KICA8ZyBjbGFzcz0ianAtaWNvbjMiIGZpbGw9IiM2MTYxNjEiPgogICAgPHBhdGggZD0iTTEyLjEsMTAuOWgtMC43bC0wLjItMC4yYzAuOC0wLjksMS4zLTIuMiwxLjMtMy41YzAtMy0yLjQtNS40LTUuNC01LjRTMS44LDQuMiwxLjgsNy4xczIuNCw1LjQsNS40LDUuNCBjMS4zLDAsMi41LTAuNSwzLjUtMS4zbDAuMiwwLjJ2MC43bDQuMSw0LjFsMS4yLTEuMkwxMi4xLDEwLjl6IE03LjEsMTAuOWMtMi4xLDAtMy43LTEuNy0zLjctMy43czEuNy0zLjcsMy43LTMuN3MzLjcsMS43LDMuNywzLjcgUzkuMiwxMC45LDcuMSwxMC45eiIvPgogIDwvZz4KPC9zdmc+Cg==);
  --jp-icon-settings: url(data:image/svg+xml;base64,PHN2ZyB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIHdpZHRoPSIxNiIgdmlld0JveD0iMCAwIDI0IDI0Ij4KICA8cGF0aCBjbGFzcz0ianAtaWNvbjMganAtaWNvbi1zZWxlY3RhYmxlIiBmaWxsPSIjNjE2MTYxIiBkPSJNMTkuNDMgMTIuOThjLjA0LS4zMi4wNy0uNjQuMDctLjk4cy0uMDMtLjY2LS4wNy0uOThsMi4xMS0xLjY1Yy4xOS0uMTUuMjQtLjQyLjEyLS42NGwtMi0zLjQ2Yy0uMTItLjIyLS4zOS0uMy0uNjEtLjIybC0yLjQ5IDFjLS41Mi0uNC0xLjA4LS43My0xLjY5LS45OGwtLjM4LTIuNjVBLjQ4OC40ODggMCAwMDE0IDJoLTRjLS4yNSAwLS40Ni4xOC0uNDkuNDJsLS4zOCAyLjY1Yy0uNjEuMjUtMS4xNy41OS0xLjY5Ljk4bC0yLjQ5LTFjLS4yMy0uMDktLjQ5IDAtLjYxLjIybC0yIDMuNDZjLS4xMy4yMi0uMDcuNDkuMTIuNjRsMi4xMSAxLjY1Yy0uMDQuMzItLjA3LjY1LS4wNy45OHMuMDMuNjYuMDcuOThsLTIuMTEgMS42NWMtLjE5LjE1LS4yNC40Mi0uMTIuNjRsMiAzLjQ2Yy4xMi4yMi4zOS4zLjYxLjIybDIuNDktMWMuNTIuNCAxLjA4LjczIDEuNjkuOThsLjM4IDIuNjVjLjAzLjI0LjI0LjQyLjQ5LjQyaDRjLjI1IDAgLjQ2LS4xOC40OS0uNDJsLjM4LTIuNjVjLjYxLS4yNSAxLjE3LS41OSAxLjY5LS45OGwyLjQ5IDFjLjIzLjA5LjQ5IDAgLjYxLS4yMmwyLTMuNDZjLjEyLS4yMi4wNy0uNDktLjEyLS42NGwtMi4xMS0xLjY1ek0xMiAxNS41Yy0xLjkzIDAtMy41LTEuNTctMy41LTMuNXMxLjU3LTMuNSAzLjUtMy41IDMuNSAxLjU3IDMuNSAzLjUtMS41NyAzLjUtMy41IDMuNXoiLz4KPC9zdmc+Cg==);
  --jp-icon-share: url(data:image/svg+xml;base64,PHN2ZyB3aWR0aD0iMTYiIHZpZXdCb3g9IjAgMCAyNCAyNCIgeG1sbnM9Imh0dHA6Ly93d3cudzMub3JnLzIwMDAvc3ZnIj4KICA8ZyBjbGFzcz0ianAtaWNvbjMiIGZpbGw9IiM2MTYxNjEiPgogICAgPHBhdGggZD0iTSAxOCAyIEMgMTYuMzU0OTkgMiAxNSAzLjM1NDk5MDQgMTUgNSBDIDE1IDUuMTkwOTUyOSAxNS4wMjE3OTEgNS4zNzcxMjI0IDE1LjA1NjY0MSA1LjU1ODU5MzggTCA3LjkyMTg3NSA5LjcyMDcwMzEgQyA3LjM5ODUzOTkgOS4yNzc4NTM5IDYuNzMyMDc3MSA5IDYgOSBDIDQuMzU0OTkwNCA5IDMgMTAuMzU0OTkgMyAxMiBDIDMgMTMuNjQ1MDEgNC4zNTQ5OTA0IDE1IDYgMTUgQyA2LjczMjA3NzEgMTUgNy4zOTg1Mzk5IDE0LjcyMjE0NiA3LjkyMTg3NSAxNC4yNzkyOTcgTCAxNS4wNTY2NDEgMTguNDM5NDUzIEMgMTUuMDIxNTU1IDE4LjYyMTUxNCAxNSAxOC44MDgzODYgMTUgMTkgQyAxNSAyMC42NDUwMSAxNi4zNTQ5OSAyMiAxOCAyMiBDIDE5LjY0NTAxIDIyIDIxIDIwLjY0NTAxIDIxIDE5IEMgMjEgMTcuMzU0OTkgMTkuNjQ1MDEgMTYgMTggMTYgQyAxNy4yNjc0OCAxNiAxNi42MDE1OTMgMTYuMjc5MzI4IDE2LjA3ODEyNSAxNi43MjI2NTYgTCA4Ljk0MzM1OTQgMTIuNTU4NTk0IEMgOC45NzgyMDk1IDEyLjM3NzEyMiA5IDEyLjE5MDk1MyA5IDEyIEMgOSAxMS44MDkwNDcgOC45NzgyMDk1IDExLjYyMjg3OCA4Ljk0MzM1OTQgMTEuNDQxNDA2IEwgMTYuMDc4MTI1IDcuMjc5Mjk2OSBDIDE2LjYwMTQ2IDcuNzIyMTQ2MSAxNy4yNjc5MjMgOCAxOCA4IEMgMTkuNjQ1MDEgOCAyMSA2LjY0NTAwOTYgMjEgNSBDIDIxIDMuMzU0OTkwNCAxOS42NDUwMSAyIDE4IDIgeiBNIDE4IDQgQyAxOC41NjQxMjkgNCAxOSA0LjQzNTg3MDYgMTkgNSBDIDE5IDUuNTY0MTI5NCAxOC41NjQxMjkgNiAxOCA2IEMgMTcuNDM1ODcxIDYgMTcgNS41NjQxMjk0IDE3IDUgQyAxNyA0LjQzNTg3MDYgMTcuNDM1ODcxIDQgMTggNCB6IE0gNiAxMSBDIDYuNTY0MTI5NCAxMSA3IDExLjQzNTg3MSA3IDEyIEMgNyAxMi41NjQxMjkgNi41NjQxMjk0IDEzIDYgMTMgQyA1LjQzNTg3MDYgMTMgNSAxMi41NjQxMjkgNSAxMiBDIDUgMTEuNDM1ODcxIDUuNDM1ODcwNiAxMSA2IDExIHogTSAxOCAxOCBDIDE4LjU2NDEyOSAxOCAxOSAxOC40MzU4NzEgMTkgMTkgQyAxOSAxOS41NjQxMjkgMTguNTY0MTI5IDIwIDE4IDIwIEMgMTcuNDM1ODcxIDIwIDE3IDE5LjU2NDEyOSAxNyAxOSBDIDE3IDE4LjQzNTg3MSAxNy40MzU4NzEgMTggMTggMTggeiIvPgogIDwvZz4KPC9zdmc+Cg==);
  --jp-icon-spreadsheet: url(data:image/svg+xml;base64,PHN2ZyB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIHdpZHRoPSIxNiIgdmlld0JveD0iMCAwIDIyIDIyIj4KICA8cGF0aCBjbGFzcz0ianAtaWNvbi1jb250cmFzdDEganAtaWNvbi1zZWxlY3RhYmxlIiBmaWxsPSIjNENBRjUwIiBkPSJNMi4yIDIuMnYxNy42aDE3LjZWMi4ySDIuMnptMTUuNCA3LjdoLTUuNVY0LjRoNS41djUuNXpNOS45IDQuNHY1LjVINC40VjQuNGg1LjV6bS01LjUgNy43aDUuNXY1LjVINC40di01LjV6bTcuNyA1LjV2LTUuNWg1LjV2NS41aC01LjV6Ii8+Cjwvc3ZnPgo=);
  --jp-icon-stop: url(data:image/svg+xml;base64,PHN2ZyBoZWlnaHQ9IjI0IiB2aWV3Qm94PSIwIDAgMjQgMjQiIHdpZHRoPSIyNCIgeG1sbnM9Imh0dHA6Ly93d3cudzMub3JnLzIwMDAvc3ZnIj4KICAgIDxnIGNsYXNzPSJqcC1pY29uMyIgZmlsbD0iIzYxNjE2MSI+CiAgICAgICAgPHBhdGggZD0iTTAgMGgyNHYyNEgweiIgZmlsbD0ibm9uZSIvPgogICAgICAgIDxwYXRoIGQ9Ik02IDZoMTJ2MTJINnoiLz4KICAgIDwvZz4KPC9zdmc+Cg==);
  --jp-icon-tab: url(data:image/svg+xml;base64,PHN2ZyB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIHdpZHRoPSIxNiIgdmlld0JveD0iMCAwIDI0IDI0Ij4KICA8ZyBjbGFzcz0ianAtaWNvbjMiIGZpbGw9IiM2MTYxNjEiPgogICAgPHBhdGggZD0iTTIxIDNIM2MtMS4xIDAtMiAuOS0yIDJ2MTRjMCAxLjEuOSAyIDIgMmgxOGMxLjEgMCAyLS45IDItMlY1YzAtMS4xLS45LTItMi0yem0wIDE2SDNWNWgxMHY0aDh2MTB6Ii8+CiAgPC9nPgo8L3N2Zz4K);
  --jp-icon-table-rows: url(data:image/svg+xml;base64,PHN2ZyBoZWlnaHQ9IjI0IiB2aWV3Qm94PSIwIDAgMjQgMjQiIHdpZHRoPSIyNCIgeG1sbnM9Imh0dHA6Ly93d3cudzMub3JnLzIwMDAvc3ZnIj4KICAgIDxnIGNsYXNzPSJqcC1pY29uMyIgZmlsbD0iIzYxNjE2MSI+CiAgICAgICAgPHBhdGggZD0iTTAgMGgyNHYyNEgweiIgZmlsbD0ibm9uZSIvPgogICAgICAgIDxwYXRoIGQ9Ik0yMSw4SDNWNGgxOFY4eiBNMjEsMTBIM3Y0aDE4VjEweiBNMjEsMTZIM3Y0aDE4VjE2eiIvPgogICAgPC9nPgo8L3N2Zz4K);
  --jp-icon-tag: url(data:image/svg+xml;base64,PHN2ZyB3aWR0aD0iMjgiIGhlaWdodD0iMjgiIHZpZXdCb3g9IjAgMCA0MyAyOCIgeG1sbnM9Imh0dHA6Ly93d3cudzMub3JnLzIwMDAvc3ZnIj4KCTxnIGNsYXNzPSJqcC1pY29uMyIgZmlsbD0iIzYxNjE2MSI+CgkJPHBhdGggZD0iTTI4LjgzMzIgMTIuMzM0TDMyLjk5OTggMTYuNTAwN0wzNy4xNjY1IDEyLjMzNEgyOC44MzMyWiIvPgoJCTxwYXRoIGQ9Ik0xNi4yMDk1IDIxLjYxMDRDMTUuNjg3MyAyMi4xMjk5IDE0Ljg0NDMgMjIuMTI5OSAxNC4zMjQ4IDIxLjYxMDRMNi45ODI5IDE0LjcyNDVDNi41NzI0IDE0LjMzOTQgNi4wODMxMyAxMy42MDk4IDYuMDQ3ODYgMTMuMDQ4MkM1Ljk1MzQ3IDExLjUyODggNi4wMjAwMiA4LjYxOTQ0IDYuMDY2MjEgNy4wNzY5NUM2LjA4MjgxIDYuNTE0NzcgNi41NTU0OCA2LjA0MzQ3IDcuMTE4MDQgNi4wMzA1NUM5LjA4ODYzIDUuOTg0NzMgMTMuMjYzOCA1LjkzNTc5IDEzLjY1MTggNi4zMjQyNUwyMS43MzY5IDEzLjYzOUMyMi4yNTYgMTQuMTU4NSAyMS43ODUxIDE1LjQ3MjQgMjEuMjYyIDE1Ljk5NDZMMTYuMjA5NSAyMS42MTA0Wk05Ljc3NTg1IDguMjY1QzkuMzM1NTEgNy44MjU2NiA4LjYyMzUxIDcuODI1NjYgOC4xODI4IDguMjY1QzcuNzQzNDYgOC43MDU3MSA3Ljc0MzQ2IDkuNDE3MzMgOC4xODI4IDkuODU2NjdDOC42MjM4MiAxMC4yOTY0IDkuMzM1ODIgMTAuMjk2NCA5Ljc3NTg1IDkuODU2NjdDMTAuMjE1NiA5LjQxNzMzIDEwLjIxNTYgOC43MDUzMyA5Ljc3NTg1IDguMjY1WiIvPgoJPC9nPgo8L3N2Zz4K);
  --jp-icon-terminal: url(data:image/svg+xml;base64,PHN2ZyB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIHdpZHRoPSIxNiIgdmlld0JveD0iMCAwIDI0IDI0IiA+CiAgICA8cmVjdCBjbGFzcz0ianAtdGVybWluYWwtaWNvbi1iYWNrZ3JvdW5kLWNvbG9yIGpwLWljb24tc2VsZWN0YWJsZSIgd2lkdGg9IjIwIiBoZWlnaHQ9IjIwIiB0cmFuc2Zvcm09InRyYW5zbGF0ZSgyIDIpIiBmaWxsPSIjMzMzMzMzIi8+CiAgICA8cGF0aCBjbGFzcz0ianAtdGVybWluYWwtaWNvbi1jb2xvciBqcC1pY29uLXNlbGVjdGFibGUtaW52ZXJzZSIgZD0iTTUuMDU2NjQgOC43NjE3MkM1LjA1NjY0IDguNTk3NjYgNS4wMzEyNSA4LjQ1MzEyIDQuOTgwNDcgOC4zMjgxMkM0LjkzMzU5IDguMTk5MjIgNC44NTU0NyA4LjA4MjAzIDQuNzQ2MDkgNy45NzY1NkM0LjY0MDYyIDcuODcxMDkgNC41IDcuNzc1MzkgNC4zMjQyMiA3LjY4OTQ1QzQuMTUyMzQgNy41OTk2MSAzLjk0MzM2IDcuNTExNzIgMy42OTcyNyA3LjQyNTc4QzMuMzAyNzMgNy4yODUxNiAyLjk0MzM2IDcuMTM2NzIgMi42MTkxNCA2Ljk4MDQ3QzIuMjk0OTIgNi44MjQyMiAyLjAxNzU4IDYuNjQyNTggMS43ODcxMSA2LjQzNTU1QzEuNTYwNTUgNi4yMjg1MiAxLjM4NDc3IDUuOTg4MjggMS4yNTk3NyA1LjcxNDg0QzEuMTM0NzcgNS40Mzc1IDEuMDcyMjcgNS4xMDkzOCAxLjA3MjI3IDQuNzMwNDdDMS4wNzIyNyA0LjM5ODQ0IDEuMTI4OTEgNC4wOTU3IDEuMjQyMTkgMy44MjIyN0MxLjM1NTQ3IDMuNTQ0OTIgMS41MTU2MiAzLjMwNDY5IDEuNzIyNjYgMy4xMDE1NkMxLjkyOTY5IDIuODk4NDQgMi4xNzk2OSAyLjczNDM3IDIuNDcyNjYgMi42MDkzOEMyLjc2NTYyIDIuNDg0MzggMy4wOTE4IDIuNDA0MyAzLjQ1MTE3IDIuMzY5MTRWMS4xMDkzOEg0LjM4ODY3VjIuMzgwODZDNC43NDAyMyAyLjQyNzczIDUuMDU2NjQgMi41MjM0NCA1LjMzNzg5IDIuNjY3OTdDNS42MTkxNCAyLjgxMjUgNS44NTc0MiAzLjAwMTk1IDYuMDUyNzMgMy4yMzYzM0M2LjI1MTk1IDMuNDY2OCA2LjQwNDMgMy43NDAyMyA2LjUwOTc3IDQuMDU2NjRDNi42MTkxNCA0LjM2OTE0IDYuNjczODMgNC43MjA3IDYuNjczODMgNS4xMTEzM0g1LjA0NDkyQzUuMDQ0OTIgNC42Mzg2NyA0LjkzNzUgNC4yODEyNSA0LjcyMjY2IDQuMDM5MDZDNC41MDc4MSAzLjc5Mjk3IDQuMjE2OCAzLjY2OTkyIDMuODQ5NjEgMy42Njk5MkMzLjY1MDM5IDMuNjY5OTIgMy40NzY1NiAzLjY5NzI3IDMuMzI4MTIgMy43NTE5NUMzLjE4MzU5IDMuODAyNzMgMy4wNjQ0NSAzLjg3Njk1IDIuOTcwNyAzLjk3NDYxQzIuODc2OTUgNC4wNjgzNiAyLjgwNjY0IDQuMTc5NjkgMi43NTk3NyA0LjMwODU5QzIuNzE2OCA0LjQzNzUgMi42OTUzMSA0LjU3ODEyIDIuNjk1MzEgNC43MzA0N0MyLjY5NTMxIDQuODgyODEgMi43MTY4IDUuMDE5NTMgMi43NTk3NyA1LjE0MDYyQzIuODA2NjQgNS4yNTc4MSAyLjg4MjgxIDUuMzY3MTkgMi45ODgyOCA1LjQ2ODc1QzMuMDk3NjYgNS41NzAzMSAzLjI0MDIzIDUuNjY3OTcgMy40MTYwMiA1Ljc2MTcyQzMuNTkxOCA1Ljg1MTU2IDMuODEwNTUgNS45NDMzNiA0LjA3MjI3IDYuMDM3MTFDNC40NjY4IDYuMTg1NTUgNC44MjQyMiA2LjMzOTg0IDUuMTQ0NTMgNi41QzUuNDY0ODQgNi42NTYyNSA1LjczODI4IDYuODM5ODQgNS45NjQ4NCA3LjA1MDc4QzYuMTk1MzEgNy4yNTc4MSA2LjM3MTA5IDcuNSA2LjQ5MjE5IDcuNzc3MzRDNi42MTcxOSA4LjA1MDc4IDYuNjc5NjkgOC4zNzUgNi42Nzk2OSA4Ljc1QzYuNjc5NjkgOS4wOTM3NSA2LjYyMzA1IDkuNDA0MyA2LjUwOTc3IDkuNjgxNjRDNi4zOTY0OCA5Ljk1NTA4IDYuMjM0MzggMTAuMTkxNCA2LjAyMzQ0IDEwLjM5MDZDNS44MTI1IDEwLjU4OTggNS41NTg1OSAxMC43NSA1LjI2MTcyIDEwLjg3MTFDNC45NjQ4NCAxMC45ODgzIDQuNjMyODEgMTEuMDY0NSA0LjI2NTYyIDExLjA5OTZWMTIuMjQ4SDMuMzMzOThWMTEuMDk5NkMzLjAwMTk1IDExLjA2ODQgMi42Nzk2OSAxMC45OTYxIDIuMzY3MTkgMTAuODgyOEMyLjA1NDY5IDEwLjc2NTYgMS43NzczNCAxMC41OTc3IDEuNTM1MTYgMTAuMzc4OUMxLjI5Njg4IDEwLjE2MDIgMS4xMDU0NyA5Ljg4NDc3IDAuOTYwOTM4IDkuNTUyNzNDMC44MTY0MDYgOS4yMTY4IDAuNzQ0MTQxIDguODE0NDUgMC43NDQxNDEgOC4zNDU3SDIuMzc4OTFDMi4zNzg5MSA4LjYyNjk1IDIuNDE5OTIgOC44NjMyOCAyLjUwMTk1IDkuMDU0NjlDMi41ODM5OCA5LjI0MjE5IDIuNjg5NDUgOS4zOTI1OCAyLjgxODM2IDkuNTA1ODZDMi45NTExNyA5LjYxNTIzIDMuMTAxNTYgOS42OTMzNiAzLjI2OTUzIDkuNzQwMjNDMy40Mzc1IDkuNzg3MTEgMy42MDkzOCA5LjgxMDU1IDMuNzg1MTYgOS44MTA1NUM0LjIwMzEyIDkuODEwNTUgNC41MTk1MyA5LjcxMjg5IDQuNzM0MzggOS41MTc1OEM0Ljk0OTIyIDkuMzIyMjcgNS4wNTY2NCA5LjA3MDMxIDUuMDU2NjQgOC43NjE3MlpNMTMuNDE4IDEyLjI3MTVIOC4wNzQyMlYxMUgxMy40MThWMTIuMjcxNVoiIHRyYW5zZm9ybT0idHJhbnNsYXRlKDMuOTUyNjQgNikiIGZpbGw9IndoaXRlIi8+Cjwvc3ZnPgo=);
  --jp-icon-text-editor: url(data:image/svg+xml;base64,PHN2ZyB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIHdpZHRoPSIxNiIgdmlld0JveD0iMCAwIDI0IDI0Ij4KICA8cGF0aCBjbGFzcz0ianAtdGV4dC1lZGl0b3ItaWNvbi1jb2xvciBqcC1pY29uLXNlbGVjdGFibGUiIGZpbGw9IiM2MTYxNjEiIGQ9Ik0xNSAxNUgzdjJoMTJ2LTJ6bTAtOEgzdjJoMTJWN3pNMyAxM2gxOHYtMkgzdjJ6bTAgOGgxOHYtMkgzdjJ6TTMgM3YyaDE4VjNIM3oiLz4KPC9zdmc+Cg==);
  --jp-icon-toc: url(data:image/svg+xml;base64,PHN2ZyB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIHdpZHRoPSIyNCIgaGVpZ2h0PSIyNCIgdmlld0JveD0iMCAwIDI0IDI0Ij4KICA8ZyBjbGFzcz0ianAtaWNvbjMganAtaWNvbi1zZWxlY3RhYmxlIiBmaWxsPSIjNjE2MTYxIj4KICAgIDxwYXRoIGQ9Ik03LDVIMjFWN0g3VjVNNywxM1YxMUgyMVYxM0g3TTQsNC41QTEuNSwxLjUgMCAwLDEgNS41LDZBMS41LDEuNSAwIDAsMSA0LDcuNUExLjUsMS41IDAgMCwxIDIuNSw2QTEuNSwxLjUgMCAwLDEgNCw0LjVNNCwxMC41QTEuNSwxLjUgMCAwLDEgNS41LDEyQTEuNSwxLjUgMCAwLDEgNCwxMy41QTEuNSwxLjUgMCAwLDEgMi41LDEyQTEuNSwxLjUgMCAwLDEgNCwxMC41TTcsMTlWMTdIMjFWMTlIN000LDE2LjVBMS41LDEuNSAwIDAsMSA1LjUsMThBMS41LDEuNSAwIDAsMSA0LDE5LjVBMS41LDEuNSAwIDAsMSAyLjUsMThBMS41LDEuNSAwIDAsMSA0LDE2LjVaIiAvPgogIDwvZz4KPC9zdmc+Cg==);
  --jp-icon-tree-view: url(data:image/svg+xml;base64,PHN2ZyBoZWlnaHQ9IjI0IiB2aWV3Qm94PSIwIDAgMjQgMjQiIHdpZHRoPSIyNCIgeG1sbnM9Imh0dHA6Ly93d3cudzMub3JnLzIwMDAvc3ZnIj4KICAgIDxnIGNsYXNzPSJqcC1pY29uMyIgZmlsbD0iIzYxNjE2MSI+CiAgICAgICAgPHBhdGggZD0iTTAgMGgyNHYyNEgweiIgZmlsbD0ibm9uZSIvPgogICAgICAgIDxwYXRoIGQ9Ik0yMiAxMVYzaC03djNIOVYzSDJ2OGg3VjhoMnYxMGg0djNoN3YtOGgtN3YzaC0yVjhoMnYzeiIvPgogICAgPC9nPgo8L3N2Zz4K);
  --jp-icon-trusted: url(data:image/svg+xml;base64,PHN2ZyBmaWxsPSJub25lIiB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIHdpZHRoPSIxNiIgdmlld0JveD0iMCAwIDI0IDI1Ij4KICAgIDxwYXRoIGNsYXNzPSJqcC1pY29uMiIgc3Ryb2tlPSIjMzMzMzMzIiBzdHJva2Utd2lkdGg9IjIiIHRyYW5zZm9ybT0idHJhbnNsYXRlKDIgMykiIGQ9Ik0xLjg2MDk0IDExLjQ0MDlDMC44MjY0NDggOC43NzAyNyAwLjg2Mzc3OSA2LjA1NzY0IDEuMjQ5MDcgNC4xOTkzMkMyLjQ4MjA2IDMuOTMzNDcgNC4wODA2OCAzLjQwMzQ3IDUuNjAxMDIgMi44NDQ5QzcuMjM1NDkgMi4yNDQ0IDguODU2NjYgMS41ODE1IDkuOTg3NiAxLjA5NTM5QzExLjA1OTcgMS41ODM0MSAxMi42MDk0IDIuMjQ0NCAxNC4yMTggMi44NDMzOUMxNS43NTAzIDMuNDEzOTQgMTcuMzk5NSAzLjk1MjU4IDE4Ljc1MzkgNC4yMTM4NUMxOS4xMzY0IDYuMDcxNzcgMTkuMTcwOSA4Ljc3NzIyIDE4LjEzOSAxMS40NDA5QzE3LjAzMDMgMTQuMzAzMiAxNC42NjY4IDE3LjE4NDQgOS45OTk5OSAxOC45MzU0QzUuMzMzMiAxNy4xODQ0IDIuOTY5NjggMTQuMzAzMiAxLjg2MDk0IDExLjQ0MDlaIi8+CiAgICA8cGF0aCBjbGFzcz0ianAtaWNvbjIiIGZpbGw9IiMzMzMzMzMiIHN0cm9rZT0iIzMzMzMzMyIgdHJhbnNmb3JtPSJ0cmFuc2xhdGUoOCA5Ljg2NzE5KSIgZD0iTTIuODYwMTUgNC44NjUzNUwwLjcyNjU0OSAyLjk5OTU5TDAgMy42MzA0NUwyLjg2MDE1IDYuMTMxNTdMOCAwLjYzMDg3Mkw3LjI3ODU3IDBMMi44NjAxNSA0Ljg2NTM1WiIvPgo8L3N2Zz4K);
  --jp-icon-undo: url(data:image/svg+xml;base64,PHN2ZyB2aWV3Qm94PSIwIDAgMjQgMjQiIHdpZHRoPSIxNiIgeG1sbnM9Imh0dHA6Ly93d3cudzMub3JnLzIwMDAvc3ZnIj4KICA8ZyBjbGFzcz0ianAtaWNvbjMiIGZpbGw9IiM2MTYxNjEiPgogICAgPHBhdGggZD0iTTEyLjUgOGMtMi42NSAwLTUuMDUuOTktNi45IDIuNkwyIDd2OWg5bC0zLjYyLTMuNjJjMS4zOS0xLjE2IDMuMTYtMS44OCA1LjEyLTEuODggMy41NCAwIDYuNTUgMi4zMSA3LjYgNS41bDIuMzctLjc4QzIxLjA4IDExLjAzIDE3LjE1IDggMTIuNSA4eiIvPgogIDwvZz4KPC9zdmc+Cg==);
  --jp-icon-user: url(data:image/svg+xml;base64,PHN2ZyB3aWR0aD0iMTYiIHZpZXdCb3g9IjAgMCAyNCAyNCIgeG1sbnM9Imh0dHA6Ly93d3cudzMub3JnLzIwMDAvc3ZnIj4KICA8ZyBjbGFzcz0ianAtaWNvbjMiIGZpbGw9IiM2MTYxNjEiPgogICAgPHBhdGggZD0iTTE2IDdhNCA0IDAgMTEtOCAwIDQgNCAwIDAxOCAwek0xMiAxNGE3IDcgMCAwMC03IDdoMTRhNyA3IDAgMDAtNy03eiIvPgogIDwvZz4KPC9zdmc+Cg==);
  --jp-icon-users: url(data:image/svg+xml;base64,PHN2ZyB3aWR0aD0iMjQiIGhlaWdodD0iMjQiIHZlcnNpb249IjEuMSIgdmlld0JveD0iMCAwIDM2IDI0IiB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciPgogPGcgY2xhc3M9ImpwLWljb24zIiB0cmFuc2Zvcm09Im1hdHJpeCgxLjczMjcgMCAwIDEuNzMyNyAtMy42MjgyIC4wOTk1NzcpIiBmaWxsPSIjNjE2MTYxIj4KICA8cGF0aCB0cmFuc2Zvcm09Im1hdHJpeCgxLjUsMCwwLDEuNSwwLC02KSIgZD0ibTEyLjE4NiA3LjUwOThjLTEuMDUzNSAwLTEuOTc1NyAwLjU2NjUtMi40Nzg1IDEuNDEwMiAwLjc1MDYxIDAuMzEyNzcgMS4zOTc0IDAuODI2NDggMS44NzMgMS40NzI3aDMuNDg2M2MwLTEuNTkyLTEuMjg4OS0yLjg4MjgtMi44ODA5LTIuODgyOHoiLz4KICA8cGF0aCBkPSJtMjAuNDY1IDIuMzg5NWEyLjE4ODUgMi4xODg1IDAgMCAxLTIuMTg4NCAyLjE4ODUgMi4xODg1IDIuMTg4NSAwIDAgMS0yLjE4ODUtMi4xODg1IDIuMTg4NSAyLjE4ODUgMCAwIDEgMi4xODg1LTIuMTg4NSAyLjE4ODUgMi4xODg1IDAgMCAxIDIuMTg4NCAyLjE4ODV6Ii8+CiAgPHBhdGggdHJhbnNmb3JtPSJtYXRyaXgoMS41LDAsMCwxLjUsMCwtNikiIGQ9Im0zLjU4OTggOC40MjE5Yy0xLjExMjYgMC0yLjAxMzcgMC45MDExMS0yLjAxMzcgMi4wMTM3aDIuODE0NWMwLjI2Nzk3LTAuMzczMDkgMC41OTA3LTAuNzA0MzUgMC45NTg5OC0wLjk3ODUyLTAuMzQ0MzMtMC42MTY4OC0xLjAwMzEtMS4wMzUyLTEuNzU5OC0xLjAzNTJ6Ii8+CiAgPHBhdGggZD0ibTYuOTE1NCA0LjYyM2ExLjUyOTQgMS41Mjk0IDAgMCAxLTEuNTI5NCAxLjUyOTQgMS41Mjk0IDEuNTI5NCAwIDAgMS0xLjUyOTQtMS41Mjk0IDEuNTI5NCAxLjUyOTQgMCAwIDEgMS41Mjk0LTEuNTI5NCAxLjUyOTQgMS41Mjk0IDAgMCAxIDEuNTI5NCAxLjUyOTR6Ii8+CiAgPHBhdGggZD0ibTYuMTM1IDEzLjUzNWMwLTMuMjM5MiAyLjYyNTktNS44NjUgNS44NjUtNS44NjUgMy4yMzkyIDAgNS44NjUgMi42MjU5IDUuODY1IDUuODY1eiIvPgogIDxjaXJjbGUgY3g9IjEyIiBjeT0iMy43Njg1IiByPSIyLjk2ODUiLz4KIDwvZz4KPC9zdmc+Cg==);
  --jp-icon-vega: url(data:image/svg+xml;base64,PHN2ZyB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIHdpZHRoPSIxNiIgdmlld0JveD0iMCAwIDIyIDIyIj4KICA8ZyBjbGFzcz0ianAtaWNvbjEganAtaWNvbi1zZWxlY3RhYmxlIiBmaWxsPSIjMjEyMTIxIj4KICAgIDxwYXRoIGQ9Ik0xMC42IDUuNGwyLjItMy4ySDIuMnY3LjNsNC02LjZ6Ii8+CiAgICA8cGF0aCBkPSJNMTUuOCAyLjJsLTQuNCA2LjZMNyA2LjNsLTQuOCA4djUuNWgxNy42VjIuMmgtNHptLTcgMTUuNEg1LjV2LTQuNGgzLjN2NC40em00LjQgMEg5LjhWOS44aDMuNHY3Ljh6bTQuNCAwaC0zLjRWNi41aDMuNHYxMS4xeiIvPgogIDwvZz4KPC9zdmc+Cg==);
  --jp-icon-word: url(data:image/svg+xml;base64,PHN2ZyB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIHdpZHRoPSIxNiIgdmlld0JveD0iMCAwIDIwIDIwIj4KIDxnIGNsYXNzPSJqcC1pY29uMiIgZmlsbD0iIzQxNDE0MSI+CiAgPHJlY3QgeD0iMiIgeT0iMiIgd2lkdGg9IjE2IiBoZWlnaHQ9IjE2Ii8+CiA8L2c+CiA8ZyBjbGFzcz0ianAtaWNvbi1hY2NlbnQyIiB0cmFuc2Zvcm09InRyYW5zbGF0ZSguNDMgLjA0MDEpIiBmaWxsPSIjZmZmIj4KICA8cGF0aCBkPSJtNC4xNCA4Ljc2cTAuMDY4Mi0xLjg5IDIuNDItMS44OSAxLjE2IDAgMS42OCAwLjQyIDAuNTY3IDAuNDEgMC41NjcgMS4xNnYzLjQ3cTAgMC40NjIgMC41MTQgMC40NjIgMC4xMDMgMCAwLjItMC4wMjMxdjAuNzE0cS0wLjM5OSAwLjEwMy0wLjY1MSAwLjEwMy0wLjQ1MiAwLTAuNjkzLTAuMjItMC4yMzEtMC4yLTAuMjg0LTAuNjYyLTAuOTU2IDAuODcyLTIgMC44NzItMC45MDMgMC0xLjQ3LTAuNDcyLTAuNTI1LTAuNDcyLTAuNTI1LTEuMjYgMC0wLjI2MiAwLjA0NTItMC40NzIgMC4wNTY3LTAuMjIgMC4xMTYtMC4zNzggMC4wNjgyLTAuMTY4IDAuMjMxLTAuMzA0IDAuMTU4LTAuMTQ3IDAuMjYyLTAuMjQyIDAuMTE2LTAuMDkxNCAwLjM2OC0wLjE2OCAwLjI2Mi0wLjA5MTQgMC4zOTktMC4xMjYgMC4xMzYtMC4wNDUyIDAuNDcyLTAuMTAzIDAuMzM2LTAuMDU3OCAwLjUwNC0wLjA3OTggMC4xNTgtMC4wMjMxIDAuNTY3LTAuMDc5OCAwLjU1Ni0wLjA2ODIgMC43NzctMC4yMjEgMC4yMi0wLjE1MiAwLjIyLTAuNDQxdi0wLjI1MnEwLTAuNDMtMC4zNTctMC42NjItMC4zMzYtMC4yMzEtMC45NzYtMC4yMzEtMC42NjIgMC0wLjk5OCAwLjI2Mi0wLjMzNiAwLjI1Mi0wLjM5OSAwLjc5OHptMS44OSAzLjY4cTAuNzg4IDAgMS4yNi0wLjQxIDAuNTA0LTAuNDIgMC41MDQtMC45MDN2LTEuMDVxLTAuMjg0IDAuMTM2LTAuODYxIDAuMjMxLTAuNTY3IDAuMDkxNC0wLjk4NyAwLjE1OC0wLjQyIDAuMDY4Mi0wLjc2NiAwLjMyNi0wLjMzNiAwLjI1Mi0wLjMzNiAwLjcwNHQwLjMwNCAwLjcwNCAwLjg2MSAwLjI1MnoiIHN0cm9rZS13aWR0aD0iMS4wNSIvPgogIDxwYXRoIGQ9Im0xMCA0LjU2aDAuOTQ1djMuMTVxMC42NTEtMC45NzYgMS44OS0wLjk3NiAxLjE2IDAgMS44OSAwLjg0IDAuNjgyIDAuODQgMC42ODIgMi4zMSAwIDEuNDctMC43MDQgMi40Mi0wLjcwNCAwLjg4Mi0xLjg5IDAuODgyLTEuMjYgMC0xLjg5LTEuMDJ2MC43NjZoLTAuODV6bTIuNjIgMy4wNHEtMC43NDYgMC0xLjE2IDAuNjQtMC40NTIgMC42My0wLjQ1MiAxLjY4IDAgMS4wNSAwLjQ1MiAxLjY4dDEuMTYgMC42M3EwLjc3NyAwIDEuMjYtMC42MyAwLjQ5NC0wLjY0IDAuNDk0LTEuNjggMC0xLjA1LTAuNDcyLTEuNjgtMC40NjItMC42NC0xLjI2LTAuNjR6IiBzdHJva2Utd2lkdGg9IjEuMDUiLz4KICA8cGF0aCBkPSJtMi43MyAxNS44IDEzLjYgMC4wMDgxYzAuMDA2OSAwIDAtMi42IDAtMi42IDAtMC4wMDc4LTEuMTUgMC0xLjE1IDAtMC4wMDY5IDAtMC4wMDgzIDEuNS0wLjAwODMgMS41LTJlLTMgLTAuMDAxNC0xMS4zLTAuMDAxNC0xMS4zLTAuMDAxNGwtMC4wMDU5Mi0xLjVjMC0wLjAwNzgtMS4xNyAwLjAwMTMtMS4xNyAwLjAwMTN6IiBzdHJva2Utd2lkdGg9Ii45NzUiLz4KIDwvZz4KPC9zdmc+Cg==);
  --jp-icon-yaml: url(data:image/svg+xml;base64,PHN2ZyB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIHdpZHRoPSIxNiIgdmlld0JveD0iMCAwIDIyIDIyIj4KICA8ZyBjbGFzcz0ianAtaWNvbi1jb250cmFzdDIganAtaWNvbi1zZWxlY3RhYmxlIiBmaWxsPSIjRDgxQjYwIj4KICAgIDxwYXRoIGQ9Ik03LjIgMTguNnYtNS40TDMgNS42aDMuM2wxLjQgMy4xYy4zLjkuNiAxLjYgMSAyLjUuMy0uOC42LTEuNiAxLTIuNWwxLjQtMy4xaDMuNGwtNC40IDcuNnY1LjVsLTIuOS0uMXoiLz4KICAgIDxjaXJjbGUgY2xhc3M9InN0MCIgY3g9IjE3LjYiIGN5PSIxNi41IiByPSIyLjEiLz4KICAgIDxjaXJjbGUgY2xhc3M9InN0MCIgY3g9IjE3LjYiIGN5PSIxMSIgcj0iMi4xIi8+CiAgPC9nPgo8L3N2Zz4K);
}

/* Icon CSS class declarations */

.jp-AddAboveIcon {
  background-image: var(--jp-icon-add-above);
}

.jp-AddBelowIcon {
  background-image: var(--jp-icon-add-below);
}

.jp-AddIcon {
  background-image: var(--jp-icon-add);
}

.jp-BellIcon {
  background-image: var(--jp-icon-bell);
}

.jp-BugDotIcon {
  background-image: var(--jp-icon-bug-dot);
}

.jp-BugIcon {
  background-image: var(--jp-icon-bug);
}

.jp-BuildIcon {
  background-image: var(--jp-icon-build);
}

.jp-CaretDownEmptyIcon {
  background-image: var(--jp-icon-caret-down-empty);
}

.jp-CaretDownEmptyThinIcon {
  background-image: var(--jp-icon-caret-down-empty-thin);
}

.jp-CaretDownIcon {
  background-image: var(--jp-icon-caret-down);
}

.jp-CaretLeftIcon {
  background-image: var(--jp-icon-caret-left);
}

.jp-CaretRightIcon {
  background-image: var(--jp-icon-caret-right);
}

.jp-CaretUpEmptyThinIcon {
  background-image: var(--jp-icon-caret-up-empty-thin);
}

.jp-CaretUpIcon {
  background-image: var(--jp-icon-caret-up);
}

.jp-CaseSensitiveIcon {
  background-image: var(--jp-icon-case-sensitive);
}

.jp-CheckIcon {
  background-image: var(--jp-icon-check);
}

.jp-CircleEmptyIcon {
  background-image: var(--jp-icon-circle-empty);
}

.jp-CircleIcon {
  background-image: var(--jp-icon-circle);
}

.jp-ClearIcon {
  background-image: var(--jp-icon-clear);
}

.jp-CloseIcon {
  background-image: var(--jp-icon-close);
}

.jp-CodeCheckIcon {
  background-image: var(--jp-icon-code-check);
}

.jp-CodeIcon {
  background-image: var(--jp-icon-code);
}

.jp-CollapseAllIcon {
  background-image: var(--jp-icon-collapse-all);
}

.jp-ConsoleIcon {
  background-image: var(--jp-icon-console);
}

.jp-CopyIcon {
  background-image: var(--jp-icon-copy);
}

.jp-CopyrightIcon {
  background-image: var(--jp-icon-copyright);
}

.jp-CutIcon {
  background-image: var(--jp-icon-cut);
}

.jp-DeleteIcon {
  background-image: var(--jp-icon-delete);
}

.jp-DownloadIcon {
  background-image: var(--jp-icon-download);
}

.jp-DuplicateIcon {
  background-image: var(--jp-icon-duplicate);
}

.jp-EditIcon {
  background-image: var(--jp-icon-edit);
}

.jp-EllipsesIcon {
  background-image: var(--jp-icon-ellipses);
}

.jp-ErrorIcon {
  background-image: var(--jp-icon-error);
}

.jp-ExpandAllIcon {
  background-image: var(--jp-icon-expand-all);
}

.jp-ExtensionIcon {
  background-image: var(--jp-icon-extension);
}

.jp-FastForwardIcon {
  background-image: var(--jp-icon-fast-forward);
}

.jp-FileIcon {
  background-image: var(--jp-icon-file);
}

.jp-FileUploadIcon {
  background-image: var(--jp-icon-file-upload);
}

.jp-FilterDotIcon {
  background-image: var(--jp-icon-filter-dot);
}

.jp-FilterIcon {
  background-image: var(--jp-icon-filter);
}

.jp-FilterListIcon {
  background-image: var(--jp-icon-filter-list);
}

.jp-FolderFavoriteIcon {
  background-image: var(--jp-icon-folder-favorite);
}

.jp-FolderIcon {
  background-image: var(--jp-icon-folder);
}

.jp-HomeIcon {
  background-image: var(--jp-icon-home);
}

.jp-Html5Icon {
  background-image: var(--jp-icon-html5);
}

.jp-ImageIcon {
  background-image: var(--jp-icon-image);
}

.jp-InfoIcon {
  background-image: var(--jp-icon-info);
}

.jp-InspectorIcon {
  background-image: var(--jp-icon-inspector);
}

.jp-JsonIcon {
  background-image: var(--jp-icon-json);
}

.jp-JuliaIcon {
  background-image: var(--jp-icon-julia);
}

.jp-JupyterFaviconIcon {
  background-image: var(--jp-icon-jupyter-favicon);
}

.jp-JupyterIcon {
  background-image: var(--jp-icon-jupyter);
}

.jp-JupyterlabWordmarkIcon {
  background-image: var(--jp-icon-jupyterlab-wordmark);
}

.jp-KernelIcon {
  background-image: var(--jp-icon-kernel);
}

.jp-KeyboardIcon {
  background-image: var(--jp-icon-keyboard);
}

.jp-LaunchIcon {
  background-image: var(--jp-icon-launch);
}

.jp-LauncherIcon {
  background-image: var(--jp-icon-launcher);
}

.jp-LineFormIcon {
  background-image: var(--jp-icon-line-form);
}

.jp-LinkIcon {
  background-image: var(--jp-icon-link);
}

.jp-ListIcon {
  background-image: var(--jp-icon-list);
}

.jp-MarkdownIcon {
  background-image: var(--jp-icon-markdown);
}

.jp-MoveDownIcon {
  background-image: var(--jp-icon-move-down);
}

.jp-MoveUpIcon {
  background-image: var(--jp-icon-move-up);
}

.jp-NewFolderIcon {
  background-image: var(--jp-icon-new-folder);
}

.jp-NotTrustedIcon {
  background-image: var(--jp-icon-not-trusted);
}

.jp-NotebookIcon {
  background-image: var(--jp-icon-notebook);
}

.jp-NumberingIcon {
  background-image: var(--jp-icon-numbering);
}

.jp-OfflineBoltIcon {
  background-image: var(--jp-icon-offline-bolt);
}

.jp-PaletteIcon {
  background-image: var(--jp-icon-palette);
}

.jp-PasteIcon {
  background-image: var(--jp-icon-paste);
}

.jp-PdfIcon {
  background-image: var(--jp-icon-pdf);
}

.jp-PythonIcon {
  background-image: var(--jp-icon-python);
}

.jp-RKernelIcon {
  background-image: var(--jp-icon-r-kernel);
}

.jp-ReactIcon {
  background-image: var(--jp-icon-react);
}

.jp-RedoIcon {
  background-image: var(--jp-icon-redo);
}

.jp-RefreshIcon {
  background-image: var(--jp-icon-refresh);
}

.jp-RegexIcon {
  background-image: var(--jp-icon-regex);
}

.jp-RunIcon {
  background-image: var(--jp-icon-run);
}

.jp-RunningIcon {
  background-image: var(--jp-icon-running);
}

.jp-SaveIcon {
  background-image: var(--jp-icon-save);
}

.jp-SearchIcon {
  background-image: var(--jp-icon-search);
}

.jp-SettingsIcon {
  background-image: var(--jp-icon-settings);
}

.jp-ShareIcon {
  background-image: var(--jp-icon-share);
}

.jp-SpreadsheetIcon {
  background-image: var(--jp-icon-spreadsheet);
}

.jp-StopIcon {
  background-image: var(--jp-icon-stop);
}

.jp-TabIcon {
  background-image: var(--jp-icon-tab);
}

.jp-TableRowsIcon {
  background-image: var(--jp-icon-table-rows);
}

.jp-TagIcon {
  background-image: var(--jp-icon-tag);
}

.jp-TerminalIcon {
  background-image: var(--jp-icon-terminal);
}

.jp-TextEditorIcon {
  background-image: var(--jp-icon-text-editor);
}

.jp-TocIcon {
  background-image: var(--jp-icon-toc);
}

.jp-TreeViewIcon {
  background-image: var(--jp-icon-tree-view);
}

.jp-TrustedIcon {
  background-image: var(--jp-icon-trusted);
}

.jp-UndoIcon {
  background-image: var(--jp-icon-undo);
}

.jp-UserIcon {
  background-image: var(--jp-icon-user);
}

.jp-UsersIcon {
  background-image: var(--jp-icon-users);
}

.jp-VegaIcon {
  background-image: var(--jp-icon-vega);
}

.jp-WordIcon {
  background-image: var(--jp-icon-word);
}

.jp-YamlIcon {
  background-image: var(--jp-icon-yaml);
}

/*-----------------------------------------------------------------------------
| Copyright (c) Jupyter Development Team.
| Distributed under the terms of the Modified BSD License.
|----------------------------------------------------------------------------*/

/**
 * (DEPRECATED) Support for consuming icons as CSS background images
 */

.jp-Icon,
.jp-MaterialIcon {
  background-position: center;
  background-repeat: no-repeat;
  background-size: 16px;
  min-width: 16px;
  min-height: 16px;
}

.jp-Icon-cover {
  background-position: center;
  background-repeat: no-repeat;
  background-size: cover;
}

/**
 * (DEPRECATED) Support for specific CSS icon sizes
 */

.jp-Icon-16 {
  background-size: 16px;
  min-width: 16px;
  min-height: 16px;
}

.jp-Icon-18 {
  background-size: 18px;
  min-width: 18px;
  min-height: 18px;
}

.jp-Icon-20 {
  background-size: 20px;
  min-width: 20px;
  min-height: 20px;
}

/*-----------------------------------------------------------------------------
| Copyright (c) Jupyter Development Team.
| Distributed under the terms of the Modified BSD License.
|----------------------------------------------------------------------------*/

.lm-TabBar .lm-TabBar-addButton {
  align-items: center;
  display: flex;
  padding: 4px;
  padding-bottom: 5px;
  margin-right: 1px;
  background-color: var(--jp-layout-color2);
}

.lm-TabBar .lm-TabBar-addButton:hover {
  background-color: var(--jp-layout-color1);
}

.lm-DockPanel-tabBar .lm-TabBar-tab {
  width: var(--jp-private-horizontal-tab-width);
}

.lm-DockPanel-tabBar .lm-TabBar-content {
  flex: unset;
}

.lm-DockPanel-tabBar[data-orientation='horizontal'] {
  flex: 1 1 auto;
}

/*-----------------------------------------------------------------------------
| Copyright (c) Jupyter Development Team.
| Distributed under the terms of the Modified BSD License.
|----------------------------------------------------------------------------*/

/**
 * Support for icons as inline SVG HTMLElements
 */

/* recolor the primary elements of an icon */
.jp-icon0[fill] {
  fill: var(--jp-inverse-layout-color0);
}

.jp-icon1[fill] {
  fill: var(--jp-inverse-layout-color1);
}

.jp-icon2[fill] {
  fill: var(--jp-inverse-layout-color2);
}

.jp-icon3[fill] {
  fill: var(--jp-inverse-layout-color3);
}

.jp-icon4[fill] {
  fill: var(--jp-inverse-layout-color4);
}

.jp-icon0[stroke] {
  stroke: var(--jp-inverse-layout-color0);
}

.jp-icon1[stroke] {
  stroke: var(--jp-inverse-layout-color1);
}

.jp-icon2[stroke] {
  stroke: var(--jp-inverse-layout-color2);
}

.jp-icon3[stroke] {
  stroke: var(--jp-inverse-layout-color3);
}

.jp-icon4[stroke] {
  stroke: var(--jp-inverse-layout-color4);
}

/* recolor the accent elements of an icon */
.jp-icon-accent0[fill] {
  fill: var(--jp-layout-color0);
}

.jp-icon-accent1[fill] {
  fill: var(--jp-layout-color1);
}

.jp-icon-accent2[fill] {
  fill: var(--jp-layout-color2);
}

.jp-icon-accent3[fill] {
  fill: var(--jp-layout-color3);
}

.jp-icon-accent4[fill] {
  fill: var(--jp-layout-color4);
}

.jp-icon-accent0[stroke] {
  stroke: var(--jp-layout-color0);
}

.jp-icon-accent1[stroke] {
  stroke: var(--jp-layout-color1);
}

.jp-icon-accent2[stroke] {
  stroke: var(--jp-layout-color2);
}

.jp-icon-accent3[stroke] {
  stroke: var(--jp-layout-color3);
}

.jp-icon-accent4[stroke] {
  stroke: var(--jp-layout-color4);
}

/* set the color of an icon to transparent */
.jp-icon-none[fill] {
  fill: none;
}

.jp-icon-none[stroke] {
  stroke: none;
}

/* brand icon colors. Same for light and dark */
.jp-icon-brand0[fill] {
  fill: var(--jp-brand-color0);
}

.jp-icon-brand1[fill] {
  fill: var(--jp-brand-color1);
}

.jp-icon-brand2[fill] {
  fill: var(--jp-brand-color2);
}

.jp-icon-brand3[fill] {
  fill: var(--jp-brand-color3);
}

.jp-icon-brand4[fill] {
  fill: var(--jp-brand-color4);
}

.jp-icon-brand0[stroke] {
  stroke: var(--jp-brand-color0);
}

.jp-icon-brand1[stroke] {
  stroke: var(--jp-brand-color1);
}

.jp-icon-brand2[stroke] {
  stroke: var(--jp-brand-color2);
}

.jp-icon-brand3[stroke] {
  stroke: var(--jp-brand-color3);
}

.jp-icon-brand4[stroke] {
  stroke: var(--jp-brand-color4);
}

/* warn icon colors. Same for light and dark */
.jp-icon-warn0[fill] {
  fill: var(--jp-warn-color0);
}

.jp-icon-warn1[fill] {
  fill: var(--jp-warn-color1);
}

.jp-icon-warn2[fill] {
  fill: var(--jp-warn-color2);
}

.jp-icon-warn3[fill] {
  fill: var(--jp-warn-color3);
}

.jp-icon-warn0[stroke] {
  stroke: var(--jp-warn-color0);
}

.jp-icon-warn1[stroke] {
  stroke: var(--jp-warn-color1);
}

.jp-icon-warn2[stroke] {
  stroke: var(--jp-warn-color2);
}

.jp-icon-warn3[stroke] {
  stroke: var(--jp-warn-color3);
}

/* icon colors that contrast well with each other and most backgrounds */
.jp-icon-contrast0[fill] {
  fill: var(--jp-icon-contrast-color0);
}

.jp-icon-contrast1[fill] {
  fill: var(--jp-icon-contrast-color1);
}

.jp-icon-contrast2[fill] {
  fill: var(--jp-icon-contrast-color2);
}

.jp-icon-contrast3[fill] {
  fill: var(--jp-icon-contrast-color3);
}

.jp-icon-contrast0[stroke] {
  stroke: var(--jp-icon-contrast-color0);
}

.jp-icon-contrast1[stroke] {
  stroke: var(--jp-icon-contrast-color1);
}

.jp-icon-contrast2[stroke] {
  stroke: var(--jp-icon-contrast-color2);
}

.jp-icon-contrast3[stroke] {
  stroke: var(--jp-icon-contrast-color3);
}

.jp-icon-dot[fill] {
  fill: var(--jp-warn-color0);
}

.jp-jupyter-icon-color[fill] {
  fill: var(--jp-jupyter-icon-color, var(--jp-warn-color0));
}

.jp-notebook-icon-color[fill] {
  fill: var(--jp-notebook-icon-color, var(--jp-warn-color0));
}

.jp-json-icon-color[fill] {
  fill: var(--jp-json-icon-color, var(--jp-warn-color1));
}

.jp-console-icon-color[fill] {
  fill: var(--jp-console-icon-color, white);
}

.jp-console-icon-background-color[fill] {
  fill: var(--jp-console-icon-background-color, var(--jp-brand-color1));
}

.jp-terminal-icon-color[fill] {
  fill: var(--jp-terminal-icon-color, var(--jp-layout-color2));
}

.jp-terminal-icon-background-color[fill] {
  fill: var(
    --jp-terminal-icon-background-color,
    var(--jp-inverse-layout-color2)
  );
}

.jp-text-editor-icon-color[fill] {
  fill: var(--jp-text-editor-icon-color, var(--jp-inverse-layout-color3));
}

.jp-inspector-icon-color[fill] {
  fill: var(--jp-inspector-icon-color, var(--jp-inverse-layout-color3));
}

/* CSS for icons in selected filebrowser listing items */
.jp-DirListing-item.jp-mod-selected .jp-icon-selectable[fill] {
  fill: #fff;
}

.jp-DirListing-item.jp-mod-selected .jp-icon-selectable-inverse[fill] {
  fill: var(--jp-brand-color1);
}

/* stylelint-disable selector-max-class, selector-max-compound-selectors */

/**
* TODO: come up with non css-hack solution for showing the busy icon on top
*  of the close icon
* CSS for complex behavior of close icon of tabs in the main area tabbar
*/
.lm-DockPanel-tabBar
  .lm-TabBar-tab.lm-mod-closable.jp-mod-dirty
  > .lm-TabBar-tabCloseIcon
  > :not(:hover)
  > .jp-icon3[fill] {
  fill: none;
}

.lm-DockPanel-tabBar
  .lm-TabBar-tab.lm-mod-closable.jp-mod-dirty
  > .lm-TabBar-tabCloseIcon
  > :not(:hover)
  > .jp-icon-busy[fill] {
  fill: var(--jp-inverse-layout-color3);
}

/* stylelint-enable selector-max-class, selector-max-compound-selectors */

/* CSS for icons in status bar */
#jp-main-statusbar .jp-mod-selected .jp-icon-selectable[fill] {
  fill: #fff;
}

#jp-main-statusbar .jp-mod-selected .jp-icon-selectable-inverse[fill] {
  fill: var(--jp-brand-color1);
}

/* special handling for splash icon CSS. While the theme CSS reloads during
   splash, the splash icon can loose theming. To prevent that, we set a
   default for its color variable */
:root {
  --jp-warn-color0: var(--md-orange-700);
}

/* not sure what to do with this one, used in filebrowser listing */
.jp-DragIcon {
  margin-right: 4px;
}

/*-----------------------------------------------------------------------------
| Copyright (c) Jupyter Development Team.
| Distributed under the terms of the Modified BSD License.
|----------------------------------------------------------------------------*/

/**
 * Support for alt colors for icons as inline SVG HTMLElements
 */

/* alt recolor the primary elements of an icon */
.jp-icon-alt .jp-icon0[fill] {
  fill: var(--jp-layout-color0);
}

.jp-icon-alt .jp-icon1[fill] {
  fill: var(--jp-layout-color1);
}

.jp-icon-alt .jp-icon2[fill] {
  fill: var(--jp-layout-color2);
}

.jp-icon-alt .jp-icon3[fill] {
  fill: var(--jp-layout-color3);
}

.jp-icon-alt .jp-icon4[fill] {
  fill: var(--jp-layout-color4);
}

.jp-icon-alt .jp-icon0[stroke] {
  stroke: var(--jp-layout-color0);
}

.jp-icon-alt .jp-icon1[stroke] {
  stroke: var(--jp-layout-color1);
}

.jp-icon-alt .jp-icon2[stroke] {
  stroke: var(--jp-layout-color2);
}

.jp-icon-alt .jp-icon3[stroke] {
  stroke: var(--jp-layout-color3);
}

.jp-icon-alt .jp-icon4[stroke] {
  stroke: var(--jp-layout-color4);
}

/* alt recolor the accent elements of an icon */
.jp-icon-alt .jp-icon-accent0[fill] {
  fill: var(--jp-inverse-layout-color0);
}

.jp-icon-alt .jp-icon-accent1[fill] {
  fill: var(--jp-inverse-layout-color1);
}

.jp-icon-alt .jp-icon-accent2[fill] {
  fill: var(--jp-inverse-layout-color2);
}

.jp-icon-alt .jp-icon-accent3[fill] {
  fill: var(--jp-inverse-layout-color3);
}

.jp-icon-alt .jp-icon-accent4[fill] {
  fill: var(--jp-inverse-layout-color4);
}

.jp-icon-alt .jp-icon-accent0[stroke] {
  stroke: var(--jp-inverse-layout-color0);
}

.jp-icon-alt .jp-icon-accent1[stroke] {
  stroke: var(--jp-inverse-layout-color1);
}

.jp-icon-alt .jp-icon-accent2[stroke] {
  stroke: var(--jp-inverse-layout-color2);
}

.jp-icon-alt .jp-icon-accent3[stroke] {
  stroke: var(--jp-inverse-layout-color3);
}

.jp-icon-alt .jp-icon-accent4[stroke] {
  stroke: var(--jp-inverse-layout-color4);
}

/*-----------------------------------------------------------------------------
| Copyright (c) Jupyter Development Team.
| Distributed under the terms of the Modified BSD License.
|----------------------------------------------------------------------------*/

.jp-icon-hoverShow:not(:hover) .jp-icon-hoverShow-content {
  display: none !important;
}

/**
 * Support for hover colors for icons as inline SVG HTMLElements
 */

/**
 * regular colors
 */

/* recolor the primary elements of an icon */
.jp-icon-hover :hover .jp-icon0-hover[fill] {
  fill: var(--jp-inverse-layout-color0);
}

.jp-icon-hover :hover .jp-icon1-hover[fill] {
  fill: var(--jp-inverse-layout-color1);
}

.jp-icon-hover :hover .jp-icon2-hover[fill] {
  fill: var(--jp-inverse-layout-color2);
}

.jp-icon-hover :hover .jp-icon3-hover[fill] {
  fill: var(--jp-inverse-layout-color3);
}

.jp-icon-hover :hover .jp-icon4-hover[fill] {
  fill: var(--jp-inverse-layout-color4);
}

.jp-icon-hover :hover .jp-icon0-hover[stroke] {
  stroke: var(--jp-inverse-layout-color0);
}

.jp-icon-hover :hover .jp-icon1-hover[stroke] {
  stroke: var(--jp-inverse-layout-color1);
}

.jp-icon-hover :hover .jp-icon2-hover[stroke] {
  stroke: var(--jp-inverse-layout-color2);
}

.jp-icon-hover :hover .jp-icon3-hover[stroke] {
  stroke: var(--jp-inverse-layout-color3);
}

.jp-icon-hover :hover .jp-icon4-hover[stroke] {
  stroke: var(--jp-inverse-layout-color4);
}

/* recolor the accent elements of an icon */
.jp-icon-hover :hover .jp-icon-accent0-hover[fill] {
  fill: var(--jp-layout-color0);
}

.jp-icon-hover :hover .jp-icon-accent1-hover[fill] {
  fill: var(--jp-layout-color1);
}

.jp-icon-hover :hover .jp-icon-accent2-hover[fill] {
  fill: var(--jp-layout-color2);
}

.jp-icon-hover :hover .jp-icon-accent3-hover[fill] {
  fill: var(--jp-layout-color3);
}

.jp-icon-hover :hover .jp-icon-accent4-hover[fill] {
  fill: var(--jp-layout-color4);
}

.jp-icon-hover :hover .jp-icon-accent0-hover[stroke] {
  stroke: var(--jp-layout-color0);
}

.jp-icon-hover :hover .jp-icon-accent1-hover[stroke] {
  stroke: var(--jp-layout-color1);
}

.jp-icon-hover :hover .jp-icon-accent2-hover[stroke] {
  stroke: var(--jp-layout-color2);
}

.jp-icon-hover :hover .jp-icon-accent3-hover[stroke] {
  stroke: var(--jp-layout-color3);
}

.jp-icon-hover :hover .jp-icon-accent4-hover[stroke] {
  stroke: var(--jp-layout-color4);
}

/* set the color of an icon to transparent */
.jp-icon-hover :hover .jp-icon-none-hover[fill] {
  fill: none;
}

.jp-icon-hover :hover .jp-icon-none-hover[stroke] {
  stroke: none;
}

/**
 * inverse colors
 */

/* inverse recolor the primary elements of an icon */
.jp-icon-hover.jp-icon-alt :hover .jp-icon0-hover[fill] {
  fill: var(--jp-layout-color0);
}

.jp-icon-hover.jp-icon-alt :hover .jp-icon1-hover[fill] {
  fill: var(--jp-layout-color1);
}

.jp-icon-hover.jp-icon-alt :hover .jp-icon2-hover[fill] {
  fill: var(--jp-layout-color2);
}

.jp-icon-hover.jp-icon-alt :hover .jp-icon3-hover[fill] {
  fill: var(--jp-layout-color3);
}

.jp-icon-hover.jp-icon-alt :hover .jp-icon4-hover[fill] {
  fill: var(--jp-layout-color4);
}

.jp-icon-hover.jp-icon-alt :hover .jp-icon0-hover[stroke] {
  stroke: var(--jp-layout-color0);
}

.jp-icon-hover.jp-icon-alt :hover .jp-icon1-hover[stroke] {
  stroke: var(--jp-layout-color1);
}

.jp-icon-hover.jp-icon-alt :hover .jp-icon2-hover[stroke] {
  stroke: var(--jp-layout-color2);
}

.jp-icon-hover.jp-icon-alt :hover .jp-icon3-hover[stroke] {
  stroke: var(--jp-layout-color3);
}

.jp-icon-hover.jp-icon-alt :hover .jp-icon4-hover[stroke] {
  stroke: var(--jp-layout-color4);
}

/* inverse recolor the accent elements of an icon */
.jp-icon-hover.jp-icon-alt :hover .jp-icon-accent0-hover[fill] {
  fill: var(--jp-inverse-layout-color0);
}

.jp-icon-hover.jp-icon-alt :hover .jp-icon-accent1-hover[fill] {
  fill: var(--jp-inverse-layout-color1);
}

.jp-icon-hover.jp-icon-alt :hover .jp-icon-accent2-hover[fill] {
  fill: var(--jp-inverse-layout-color2);
}

.jp-icon-hover.jp-icon-alt :hover .jp-icon-accent3-hover[fill] {
  fill: var(--jp-inverse-layout-color3);
}

.jp-icon-hover.jp-icon-alt :hover .jp-icon-accent4-hover[fill] {
  fill: var(--jp-inverse-layout-color4);
}

.jp-icon-hover.jp-icon-alt :hover .jp-icon-accent0-hover[stroke] {
  stroke: var(--jp-inverse-layout-color0);
}

.jp-icon-hover.jp-icon-alt :hover .jp-icon-accent1-hover[stroke] {
  stroke: var(--jp-inverse-layout-color1);
}

.jp-icon-hover.jp-icon-alt :hover .jp-icon-accent2-hover[stroke] {
  stroke: var(--jp-inverse-layout-color2);
}

.jp-icon-hover.jp-icon-alt :hover .jp-icon-accent3-hover[stroke] {
  stroke: var(--jp-inverse-layout-color3);
}

.jp-icon-hover.jp-icon-alt :hover .jp-icon-accent4-hover[stroke] {
  stroke: var(--jp-inverse-layout-color4);
}

/*-----------------------------------------------------------------------------
| Copyright (c) Jupyter Development Team.
| Distributed under the terms of the Modified BSD License.
|----------------------------------------------------------------------------*/

.jp-IFrame {
  width: 100%;
  height: 100%;
}

.jp-IFrame > iframe {
  border: none;
}

/*
When drag events occur, `lm-mod-override-cursor` is added to the body.
Because iframes steal all cursor events, the following two rules are necessary
to suppress pointer events while resize drags are occurring. There may be a
better solution to this problem.
*/
body.lm-mod-override-cursor .jp-IFrame {
  position: relative;
}

body.lm-mod-override-cursor .jp-IFrame::before {
  content: '';
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: transparent;
}

/*-----------------------------------------------------------------------------
| Copyright (c) 2014-2016, Jupyter Development Team.
|
| Distributed under the terms of the Modified BSD License.
|----------------------------------------------------------------------------*/

.jp-HoverBox {
  position: fixed;
}

/*-----------------------------------------------------------------------------
| Copyright (c) Jupyter Development Team.
| Distributed under the terms of the Modified BSD License.
|----------------------------------------------------------------------------*/

.jp-FormGroup-content fieldset {
  border: none;
  padding: 0;
  min-width: 0;
  width: 100%;
}

/* stylelint-disable selector-max-type */

.jp-FormGroup-content fieldset .jp-inputFieldWrapper input,
.jp-FormGroup-content fieldset .jp-inputFieldWrapper select,
.jp-FormGroup-content fieldset .jp-inputFieldWrapper textarea {
  font-size: var(--jp-content-font-size2);
  border-color: var(--jp-input-border-color);
  border-style: solid;
  border-radius: var(--jp-border-radius);
  border-width: 1px;
  padding: 6px 8px;
  background: none;
  color: var(--jp-ui-font-color0);
  height: inherit;
}

.jp-FormGroup-content fieldset input[type='checkbox'] {
  position: relative;
  top: 2px;
  margin-left: 0;
}

.jp-FormGroup-content button.jp-mod-styled {
  cursor: pointer;
}

.jp-FormGroup-content .checkbox label {
  cursor: pointer;
  font-size: var(--jp-content-font-size1);
}

.jp-FormGroup-content .jp-root > fieldset > legend {
  display: none;
}

.jp-FormGroup-content .jp-root > fieldset > p {
  display: none;
}

/** copy of `input.jp-mod-styled:focus` style */
.jp-FormGroup-content fieldset input:focus,
.jp-FormGroup-content fieldset select:focus {
  -moz-outline-radius: unset;
  outline: var(--jp-border-width) solid var(--md-blue-500);
  outline-offset: -1px;
  box-shadow: inset 0 0 4px var(--md-blue-300);
}

.jp-FormGroup-content fieldset input:hover:not(:focus),
.jp-FormGroup-content fieldset select:hover:not(:focus) {
  background-color: var(--jp-border-color2);
}

/* stylelint-enable selector-max-type */

.jp-FormGroup-content .checkbox .field-description {
  /* Disable default description field for checkbox:
   because other widgets do not have description fields,
   we add descriptions to each widget on the field level.
  */
  display: none;
}

.jp-FormGroup-content #root__description {
  display: none;
}

.jp-FormGroup-content .jp-modifiedIndicator {
  width: 5px;
  background-color: var(--jp-brand-color2);
  margin-top: 0;
  margin-left: calc(var(--jp-private-settingeditor-modifier-indent) * -1);
  flex-shrink: 0;
}

.jp-FormGroup-content .jp-modifiedIndicator.jp-errorIndicator {
  background-color: var(--jp-error-color0);
  margin-right: 0.5em;
}

/* RJSF ARRAY style */

.jp-arrayFieldWrapper legend {
  font-size: var(--jp-content-font-size2);
  color: var(--jp-ui-font-color0);
  flex-basis: 100%;
  padding: 4px 0;
  font-weight: var(--jp-content-heading-font-weight);
  border-bottom: 1px solid var(--jp-border-color2);
}

.jp-arrayFieldWrapper .field-description {
  padding: 4px 0;
  white-space: pre-wrap;
}

.jp-arrayFieldWrapper .array-item {
  width: 100%;
  border: 1px solid var(--jp-border-color2);
  border-radius: 4px;
  margin: 4px;
}

.jp-ArrayOperations {
  display: flex;
  margin-left: 8px;
}

.jp-ArrayOperationsButton {
  margin: 2px;
}

.jp-ArrayOperationsButton .jp-icon3[fill] {
  fill: var(--jp-ui-font-color0);
}

button.jp-ArrayOperationsButton.jp-mod-styled:disabled {
  cursor: not-allowed;
  opacity: 0.5;
}

/* RJSF form validation error */

.jp-FormGroup-content .validationErrors {
  color: var(--jp-error-color0);
}

/* Hide panel level error as duplicated the field level error */
.jp-FormGroup-content .panel.errors {
  display: none;
}

/* RJSF normal content (settings-editor) */

.jp-FormGroup-contentNormal {
  display: flex;
  align-items: center;
  flex-wrap: wrap;
}

.jp-FormGroup-contentNormal .jp-FormGroup-contentItem {
  margin-left: 7px;
  color: var(--jp-ui-font-color0);
}

.jp-FormGroup-contentNormal .jp-FormGroup-description {
  flex-basis: 100%;
  padding: 4px 7px;
}

.jp-FormGroup-contentNormal .jp-FormGroup-default {
  flex-basis: 100%;
  padding: 4px 7px;
}

.jp-FormGroup-contentNormal .jp-FormGroup-fieldLabel {
  font-size: var(--jp-content-font-size1);
  font-weight: normal;
  min-width: 120px;
}

.jp-FormGroup-contentNormal fieldset:not(:first-child) {
  margin-left: 7px;
}

.jp-FormGroup-contentNormal .field-array-of-string .array-item {
  /* Display `jp-ArrayOperations` buttons side-by-side with content except
    for small screens where flex-wrap will place them one below the other.
  */
  display: flex;
  align-items: center;
  flex-wrap: wrap;
}

.jp-FormGroup-contentNormal .jp-objectFieldWrapper .form-group {
  padding: 2px 8px 2px var(--jp-private-settingeditor-modifier-indent);
  margin-top: 2px;
}

/* RJSF compact content (metadata-form) */

.jp-FormGroup-content.jp-FormGroup-contentCompact {
  width: 100%;
}

.jp-FormGroup-contentCompact .form-group {
  display: flex;
  padding: 0.5em 0.2em 0.5em 0;
}

.jp-FormGroup-contentCompact
  .jp-FormGroup-compactTitle
  .jp-FormGroup-description {
  font-size: var(--jp-ui-font-size1);
  color: var(--jp-ui-font-color2);
}

.jp-FormGroup-contentCompact .jp-FormGroup-fieldLabel {
  padding-bottom: 0.3em;
}

.jp-FormGroup-contentCompact .jp-inputFieldWrapper .form-control {
  width: 100%;
  box-sizing: border-box;
}

.jp-FormGroup-contentCompact .jp-arrayFieldWrapper .jp-FormGroup-compactTitle {
  padding-bottom: 7px;
}

.jp-FormGroup-contentCompact
  .jp-objectFieldWrapper
  .jp-objectFieldWrapper
  .form-group {
  padding: 2px 8px 2px var(--jp-private-settingeditor-modifier-indent);
  margin-top: 2px;
}

.jp-FormGroup-contentCompact ul.error-detail {
  margin-block-start: 0.5em;
  margin-block-end: 0.5em;
  padding-inline-start: 1em;
}

/*
 * Copyright (c) Jupyter Development Team.
 * Distributed under the terms of the Modified BSD License.
 */

.jp-SidePanel {
  display: flex;
  flex-direction: column;
  min-width: var(--jp-sidebar-min-width);
  overflow-y: auto;
  color: var(--jp-ui-font-color1);
  background: var(--jp-layout-color1);
  font-size: var(--jp-ui-font-size1);
}

.jp-SidePanel-header {
  flex: 0 0 auto;
  display: flex;
  border-bottom: var(--jp-border-width) solid var(--jp-border-color2);
  font-size: var(--jp-ui-font-size0);
  font-weight: 600;
  letter-spacing: 1px;
  margin: 0;
  padding: 2px;
  text-transform: uppercase;
}

.jp-SidePanel-toolbar {
  flex: 0 0 auto;
}

.jp-SidePanel-content {
  flex: 1 1 auto;
}

.jp-SidePanel-toolbar,
.jp-AccordionPanel-toolbar {
  height: var(--jp-private-toolbar-height);
}

.jp-SidePanel-toolbar.jp-Toolbar-micro {
  display: none;
}

.lm-AccordionPanel .jp-AccordionPanel-title {
  box-sizing: border-box;
  line-height: 25px;
  margin: 0;
  display: flex;
  align-items: center;
  background: var(--jp-layout-color1);
  color: var(--jp-ui-font-color1);
  border-bottom: var(--jp-border-width) solid var(--jp-toolbar-border-color);
  box-shadow: var(--jp-toolbar-box-shadow);
  font-size: var(--jp-ui-font-size0);
}

.jp-AccordionPanel-title {
  cursor: pointer;
  user-select: none;
  -moz-user-select: none;
  -webkit-user-select: none;
  text-transform: uppercase;
}

.lm-AccordionPanel[data-orientation='horizontal'] > .jp-AccordionPanel-title {
  /* Title is rotated for horizontal accordion panel using CSS */
  display: block;
  transform-origin: top left;
  transform: rotate(-90deg) translate(-100%);
}

.jp-AccordionPanel-title .lm-AccordionPanel-titleLabel {
  user-select: none;
  text-overflow: ellipsis;
  white-space: nowrap;
  overflow: hidden;
}

.jp-AccordionPanel-title .lm-AccordionPanel-titleCollapser {
  transform: rotate(-90deg);
  margin: auto 0;
  height: 16px;
}

.jp-AccordionPanel-title.lm-mod-expanded .lm-AccordionPanel-titleCollapser {
  transform: rotate(0deg);
}

.lm-AccordionPanel .jp-AccordionPanel-toolbar {
  background: none;
  box-shadow: none;
  border: none;
  margin-left: auto;
}

.lm-AccordionPanel .lm-SplitPanel-handle:hover {
  background: var(--jp-layout-color3);
}

.jp-text-truncated {
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

/*-----------------------------------------------------------------------------
| Copyright (c) 2017, Jupyter Development Team.
|
| Distributed under the terms of the Modified BSD License.
|----------------------------------------------------------------------------*/

.jp-Spinner {
  position: absolute;
  display: flex;
  justify-content: center;
  align-items: center;
  z-index: 10;
  left: 0;
  top: 0;
  width: 100%;
  height: 100%;
  background: var(--jp-layout-color0);
  outline: none;
}

.jp-SpinnerContent {
  font-size: 10px;
  margin: 50px auto;
  text-indent: -9999em;
  width: 3em;
  height: 3em;
  border-radius: 50%;
  background: var(--jp-brand-color3);
  background: linear-gradient(
    to right,
    #f37626 10%,
    rgba(255, 255, 255, 0) 42%
  );
  position: relative;
  animation: load3 1s infinite linear, fadeIn 1s;
}

.jp-SpinnerContent::before {
  width: 50%;
  height: 50%;
  background: #f37626;
  border-radius: 100% 0 0;
  position: absolute;
  top: 0;
  left: 0;
  content: '';
}

.jp-SpinnerContent::after {
  background: var(--jp-layout-color0);
  width: 75%;
  height: 75%;
  border-radius: 50%;
  content: '';
  margin: auto;
  position: absolute;
  top: 0;
  left: 0;
  bottom: 0;
  right: 0;
}

@keyframes fadeIn {
  0% {
    opacity: 0;
  }

  100% {
    opacity: 1;
  }
}

@keyframes load3 {
  0% {
    transform: rotate(0deg);
  }

  100% {
    transform: rotate(360deg);
  }
}

/*-----------------------------------------------------------------------------
| Copyright (c) 2014-2017, Jupyter Development Team.
|
| Distributed under the terms of the Modified BSD License.
|----------------------------------------------------------------------------*/

button.jp-mod-styled {
  font-size: var(--jp-ui-font-size1);
  color: var(--jp-ui-font-color0);
  border: none;
  box-sizing: border-box;
  text-align: center;
  line-height: 32px;
  height: 32px;
  padding: 0 12px;
  letter-spacing: 0.8px;
  outline: none;
  appearance: none;
  -webkit-appearance: none;
  -moz-appearance: none;
}

input.jp-mod-styled {
  background: var(--jp-input-background);
  height: 28px;
  box-sizing: border-box;
  border: var(--jp-border-width) solid var(--jp-border-color1);
  padding-left: 7px;
  padding-right: 7px;
  font-size: var(--jp-ui-font-size2);
  color: var(--jp-ui-font-color0);
  outline: none;
  appearance: none;
  -webkit-appearance: none;
  -moz-appearance: none;
}

input[type='checkbox'].jp-mod-styled {
  appearance: checkbox;
  -webkit-appearance: checkbox;
  -moz-appearance: checkbox;
  height: auto;
}

input.jp-mod-styled:focus {
  border: var(--jp-border-width) solid var(--md-blue-500);
  box-shadow: inset 0 0 4px var(--md-blue-300);
}

.jp-select-wrapper {
  display: flex;
  position: relative;
  flex-direction: column;
  padding: 1px;
  background-color: var(--jp-layout-color1);
  box-sizing: border-box;
  margin-bottom: 12px;
}

.jp-select-wrapper:not(.multiple) {
  height: 28px;
}

.jp-select-wrapper.jp-mod-focused select.jp-mod-styled {
  border: var(--jp-border-width) solid var(--jp-input-active-border-color);
  box-shadow: var(--jp-input-box-shadow);
  background-color: var(--jp-input-active-background);
}

select.jp-mod-styled:hover {
  cursor: pointer;
  color: var(--jp-ui-font-color0);
  background-color: var(--jp-input-hover-background);
  box-shadow: inset 0 0 1px rgba(0, 0, 0, 0.5);
}

select.jp-mod-styled {
  flex: 1 1 auto;
  width: 100%;
  font-size: var(--jp-ui-font-size2);
  background: var(--jp-input-background);
  color: var(--jp-ui-font-color0);
  padding: 0 25px 0 8px;
  border: var(--jp-border-width) solid var(--jp-input-border-color);
  border-radius: 0;
  outline: none;
  appearance: none;
  -webkit-appearance: none;
  -moz-appearance: none;
}

select.jp-mod-styled:not([multiple]) {
  height: 32px;
}

select.jp-mod-styled[multiple] {
  max-height: 200px;
  overflow-y: auto;
}

/*-----------------------------------------------------------------------------
| Copyright (c) Jupyter Development Team.
| Distributed under the terms of the Modified BSD License.
|----------------------------------------------------------------------------*/

.jp-switch {
  display: flex;
  align-items: center;
  padding-left: 4px;
  padding-right: 4px;
  font-size: var(--jp-ui-font-size1);
  background-color: transparent;
  color: var(--jp-ui-font-color1);
  border: none;
  height: 20px;
}

.jp-switch:hover {
  background-color: var(--jp-layout-color2);
}

.jp-switch-label {
  margin-right: 5px;
  font-family: var(--jp-ui-font-family);
}

.jp-switch-track {
  cursor: pointer;
  background-color: var(--jp-switch-color, var(--jp-border-color1));
  -webkit-transition: 0.4s;
  transition: 0.4s;
  border-radius: 34px;
  height: 16px;
  width: 35px;
  position: relative;
}

.jp-switch-track::before {
  content: '';
  position: absolute;
  height: 10px;
  width: 10px;
  margin: 3px;
  left: 0;
  background-color: var(--jp-ui-inverse-font-color1);
  -webkit-transition: 0.4s;
  transition: 0.4s;
  border-radius: 50%;
}

.jp-switch[aria-checked='true'] .jp-switch-track {
  background-color: var(--jp-switch-true-position-color, var(--jp-warn-color0));
}

.jp-switch[aria-checked='true'] .jp-switch-track::before {
  /* track width (35) - margins (3 + 3) - thumb width (10) */
  left: 19px;
}

/*-----------------------------------------------------------------------------
| Copyright (c) 2014-2016, Jupyter Development Team.
|
| Distributed under the terms of the Modified BSD License.
|----------------------------------------------------------------------------*/

:root {
  --jp-private-toolbar-height: calc(
    28px + var(--jp-border-width)
  ); /* leave 28px for content */
}

.jp-Toolbar {
  color: var(--jp-ui-font-color1);
  flex: 0 0 auto;
  display: flex;
  flex-direction: row;
  border-bottom: var(--jp-border-width) solid var(--jp-toolbar-border-color);
  box-shadow: var(--jp-toolbar-box-shadow);
  background: var(--jp-toolbar-background);
  min-height: var(--jp-toolbar-micro-height);
  padding: 2px;
  z-index: 8;
  overflow-x: hidden;
}

/* Toolbar items */

.jp-Toolbar > .jp-Toolbar-item.jp-Toolbar-spacer {
  flex-grow: 1;
  flex-shrink: 1;
}

.jp-Toolbar-item.jp-Toolbar-kernelStatus {
  display: inline-block;
  width: 32px;
  background-repeat: no-repeat;
  background-position: center;
  background-size: 16px;
}

.jp-Toolbar > .jp-Toolbar-item {
  flex: 0 0 auto;
  display: flex;
  padding-left: 1px;
  padding-right: 1px;
  font-size: var(--jp-ui-font-size1);
  line-height: var(--jp-private-toolbar-height);
  height: 100%;
}

/* Toolbar buttons */

/* This is the div we use to wrap the react component into a Widget */
div.jp-ToolbarButton {
  color: transparent;
  border: none;
  box-sizing: border-box;
  outline: none;
  appearance: none;
  -webkit-appearance: none;
  -moz-appearance: none;
  padding: 0;
  margin: 0;
}

button.jp-ToolbarButtonComponent {
  background: var(--jp-layout-color1);
  border: none;
  box-sizing: border-box;
  outline: none;
  appearance: none;
  -webkit-appearance: none;
  -moz-appearance: none;
  padding: 0 6px;
  margin: 0;
  height: 24px;
  border-radius: var(--jp-border-radius);
  display: flex;
  align-items: center;
  text-align: center;
  font-size: 14px;
  min-width: unset;
  min-height: unset;
}

button.jp-ToolbarButtonComponent:disabled {
  opacity: 0.4;
}

button.jp-ToolbarButtonComponent > span {
  padding: 0;
  flex: 0 0 auto;
}

button.jp-ToolbarButtonComponent .jp-ToolbarButtonComponent-label {
  font-size: var(--jp-ui-font-size1);
  line-height: 100%;
  padding-left: 2px;
  color: var(--jp-ui-font-color1);
  font-family: var(--jp-ui-font-family);
}

#jp-main-dock-panel[data-mode='single-document']
  .jp-MainAreaWidget
  > .jp-Toolbar.jp-Toolbar-micro {
  padding: 0;
  min-height: 0;
}

#jp-main-dock-panel[data-mode='single-document']
  .jp-MainAreaWidget
  > .jp-Toolbar {
  border: none;
  box-shadow: none;
}

/*
 * Copyright (c) Jupyter Development Team.
 * Distributed under the terms of the Modified BSD License.
 */

.jp-WindowedPanel-outer {
  position: relative;
  overflow-y: auto;
}

.jp-WindowedPanel-inner {
  position: relative;
}

.jp-WindowedPanel-window {
  position: absolute;
  left: 0;
  right: 0;
  overflow: visible;
}

/*-----------------------------------------------------------------------------
| Copyright (c) Jupyter Development Team.
| Distributed under the terms of the Modified BSD License.
|----------------------------------------------------------------------------*/

/* Sibling imports */

body {
  color: var(--jp-ui-font-color1);
  font-size: var(--jp-ui-font-size1);
}

/* Disable native link decoration styles everywhere outside of dialog boxes */
a {
  text-decoration: unset;
  color: unset;
}

a:hover {
  text-decoration: unset;
  color: unset;
}

/* Accessibility for links inside dialog box text */
.jp-Dialog-content a {
  text-decoration: revert;
  color: var(--jp-content-link-color);
}

.jp-Dialog-content a:hover {
  text-decoration: revert;
}

/* Styles for ui-components */
.jp-Button {
  color: var(--jp-ui-font-color2);
  border-radius: var(--jp-border-radius);
  padding: 0 12px;
  font-size: var(--jp-ui-font-size1);

  /* Copy from blueprint 3 */
  display: inline-flex;
  flex-direction: row;
  border: none;
  cursor: pointer;
  align-items: center;
  justify-content: center;
  text-align: left;
  vertical-align: middle;
  min-height: 30px;
  min-width: 30px;
}

.jp-Button:disabled {
  cursor: not-allowed;
}

.jp-Button:empty {
  padding: 0 !important;
}

.jp-Button.jp-mod-small {
  min-height: 24px;
  min-width: 24px;
  font-size: 12px;
  padding: 0 7px;
}

/* Use our own theme for hover styles */
.jp-Button.jp-mod-minimal:hover {
  background-color: var(--jp-layout-color2);
}

.jp-Button.jp-mod-minimal {
  background: none;
}

.jp-InputGroup {
  display: block;
  position: relative;
}

.jp-InputGroup input {
  box-sizing: border-box;
  border: none;
  border-radius: 0;
  background-color: transparent;
  color: var(--jp-ui-font-color0);
  box-shadow: inset 0 0 0 var(--jp-border-width) var(--jp-input-border-color);
  padding-bottom: 0;
  padding-top: 0;
  padding-left: 10px;
  padding-right: 28px;
  position: relative;
  width: 100%;
  -webkit-appearance: none;
  -moz-appearance: none;
  appearance: none;
  font-size: 14px;
  font-weight: 400;
  height: 30px;
  line-height: 30px;
  outline: none;
  vertical-align: middle;
}

.jp-InputGroup input:focus {
  box-shadow: inset 0 0 0 var(--jp-border-width)
      var(--jp-input-active-box-shadow-color),
    inset 0 0 0 3px var(--jp-input-active-box-shadow-color);
}

.jp-InputGroup input:disabled {
  cursor: not-allowed;
  resize: block;
  background-color: var(--jp-layout-color2);
  color: var(--jp-ui-font-color2);
}

.jp-InputGroup input:disabled ~ span {
  cursor: not-allowed;
  color: var(--jp-ui-font-color2);
}

.jp-InputGroup input::placeholder,
input::placeholder {
  color: var(--jp-ui-font-color2);
}

.jp-InputGroupAction {
  position: absolute;
  bottom: 1px;
  right: 0;
  padding: 6px;
}

.jp-HTMLSelect.jp-DefaultStyle select {
  background-color: initial;
  border: none;
  border-radius: 0;
  box-shadow: none;
  color: var(--jp-ui-font-color0);
  display: block;
  font-size: var(--jp-ui-font-size1);
  font-family: var(--jp-ui-font-family);
  height: 24px;
  line-height: 14px;
  padding: 0 25px 0 10px;
  text-align: left;
  -moz-appearance: none;
  -webkit-appearance: none;
}

.jp-HTMLSelect.jp-DefaultStyle select:disabled {
  background-color: var(--jp-layout-color2);
  color: var(--jp-ui-font-color2);
  cursor: not-allowed;
  resize: block;
}

.jp-HTMLSelect.jp-DefaultStyle select:disabled ~ span {
  cursor: not-allowed;
}

/* Use our own theme for hover and option styles */
/* stylelint-disable-next-line selector-max-type */
.jp-HTMLSelect.jp-DefaultStyle select:hover,
.jp-HTMLSelect.jp-DefaultStyle select > option {
  background-color: var(--jp-layout-color2);
  color: var(--jp-ui-font-color0);
}

select {
  box-sizing: border-box;
}

/*-----------------------------------------------------------------------------
| Copyright (c) Jupyter Development Team.
| Distributed under the terms of the Modified BSD License.
|----------------------------------------------------------------------------*/

/*-----------------------------------------------------------------------------
| Styles
|----------------------------------------------------------------------------*/

.jp-StatusBar-Widget {
  display: flex;
  align-items: center;
  background: var(--jp-layout-color2);
  min-height: var(--jp-statusbar-height);
  justify-content: space-between;
  padding: 0 10px;
}

.jp-StatusBar-Left {
  display: flex;
  align-items: center;
  flex-direction: row;
}

.jp-StatusBar-Middle {
  display: flex;
  align-items: center;
}

.jp-StatusBar-Right {
  display: flex;
  align-items: center;
  flex-direction: row-reverse;
}

.jp-StatusBar-Item {
  max-height: var(--jp-statusbar-height);
  margin: 0 2px;
  height: var(--jp-statusbar-height);
  white-space: nowrap;
  text-overflow: ellipsis;
  color: var(--jp-ui-font-color1);
  padding: 0 6px;
}

.jp-mod-highlighted:hover {
  background-color: var(--jp-layout-color3);
}

.jp-mod-clicked {
  background-color: var(--jp-brand-color1);
}

.jp-mod-clicked:hover {
  background-color: var(--jp-brand-color0);
}

.jp-mod-clicked .jp-StatusBar-TextItem {
  color: var(--jp-ui-inverse-font-color1);
}

.jp-StatusBar-HoverItem {
  box-shadow: '0px 4px 4px rgba(0, 0, 0, 0.25)';
}

.jp-StatusBar-TextItem {
  font-size: var(--jp-ui-font-size1);
  font-family: var(--jp-ui-font-family);
  line-height: 24px;
  color: var(--jp-ui-font-color1);
}

.jp-StatusBar-GroupItem {
  display: flex;
  align-items: center;
  flex-direction: row;
}

.jp-Statusbar-ProgressCircle svg {
  display: block;
  margin: 0 auto;
  width: 16px;
  height: 24px;
  align-self: normal;
}

.jp-Statusbar-ProgressCircle path {
  fill: var(--jp-inverse-layout-color3);
}

.jp-Statusbar-ProgressBar-progress-bar {
  height: 10px;
  width: 100px;
  border: solid 0.25px var(--jp-brand-color2);
  border-radius: 3px;
  overflow: hidden;
  align-self: center;
}

.jp-Statusbar-ProgressBar-progress-bar > div {
  background-color: var(--jp-brand-color2);
  background-image: linear-gradient(
    -45deg,
    rgba(255, 255, 255, 0.2) 25%,
    transparent 25%,
    transparent 50%,
    rgba(255, 255, 255, 0.2) 50%,
    rgba(255, 255, 255, 0.2) 75%,
    transparent 75%,
    transparent
  );
  background-size: 40px 40px;
  float: left;
  width: 0%;
  height: 100%;
  font-size: 12px;
  line-height: 14px;
  color: #fff;
  text-align: center;
  animation: jp-Statusbar-ExecutionTime-progress-bar 2s linear infinite;
}

.jp-Statusbar-ProgressBar-progress-bar p {
  color: var(--jp-ui-font-color1);
  font-family: var(--jp-ui-font-family);
  font-size: var(--jp-ui-font-size1);
  line-height: 10px;
  width: 100px;
}

@keyframes jp-Statusbar-ExecutionTime-progress-bar {
  0% {
    background-position: 0 0;
  }

  100% {
    background-position: 40px 40px;
  }
}

/*-----------------------------------------------------------------------------
| Copyright (c) Jupyter Development Team.
| Distributed under the terms of the Modified BSD License.
|----------------------------------------------------------------------------*/

/*-----------------------------------------------------------------------------
| Variables
|----------------------------------------------------------------------------*/

:root {
  --jp-private-commandpalette-search-height: 28px;
}

/*-----------------------------------------------------------------------------
| Overall styles
|----------------------------------------------------------------------------*/

.lm-CommandPalette {
  padding-bottom: 0;
  color: var(--jp-ui-font-color1);
  background: var(--jp-layout-color1);

  /* This is needed so that all font sizing of children done in ems is
   * relative to this base size */
  font-size: var(--jp-ui-font-size1);
}

/*-----------------------------------------------------------------------------
| Modal variant
|----------------------------------------------------------------------------*/

.jp-ModalCommandPalette {
  position: absolute;
  z-index: 10000;
  top: 38px;
  left: 30%;
  margin: 0;
  padding: 4px;
  width: 40%;
  box-shadow: var(--jp-elevation-z4);
  border-radius: 4px;
  background: var(--jp-layout-color0);
}

.jp-ModalCommandPalette .lm-CommandPalette {
  max-height: 40vh;
}

.jp-ModalCommandPalette .lm-CommandPalette .lm-close-icon::after {
  display: none;
}

.jp-ModalCommandPalette .lm-CommandPalette .lm-CommandPalette-header {
  display: none;
}

.jp-ModalCommandPalette .lm-CommandPalette .lm-CommandPalette-item {
  margin-left: 4px;
  margin-right: 4px;
}

.jp-ModalCommandPalette
  .lm-CommandPalette
  .lm-CommandPalette-item.lm-mod-disabled {
  display: none;
}

/*-----------------------------------------------------------------------------
| Search
|----------------------------------------------------------------------------*/

.lm-CommandPalette-search {
  padding: 4px;
  background-color: var(--jp-layout-color1);
  z-index: 2;
}

.lm-CommandPalette-wrapper {
  overflow: overlay;
  padding: 0 9px;
  background-color: var(--jp-input-active-background);
  height: 30px;
  box-shadow: inset 0 0 0 var(--jp-border-width) var(--jp-input-border-color);
}

.lm-CommandPalette.lm-mod-focused .lm-CommandPalette-wrapper {
  box-shadow: inset 0 0 0 1px var(--jp-input-active-box-shadow-color),
    inset 0 0 0 3px var(--jp-input-active-box-shadow-color);
}

.jp-SearchIconGroup {
  color: white;
  background-color: var(--jp-brand-color1);
  position: absolute;
  top: 4px;
  right: 4px;
  padding: 5px 5px 1px;
}

.jp-SearchIconGroup svg {
  height: 20px;
  width: 20px;
}

.jp-SearchIconGroup .jp-icon3[fill] {
  fill: var(--jp-layout-color0);
}

.lm-CommandPalette-input {
  background: transparent;
  width: calc(100% - 18px);
  float: left;
  border: none;
  outline: none;
  font-size: var(--jp-ui-font-size1);
  color: var(--jp-ui-font-color0);
  line-height: var(--jp-private-commandpalette-search-height);
}

.lm-CommandPalette-input::-webkit-input-placeholder,
.lm-CommandPalette-input::-moz-placeholder,
.lm-CommandPalette-input:-ms-input-placeholder {
  color: var(--jp-ui-font-color2);
  font-size: var(--jp-ui-font-size1);
}

/*-----------------------------------------------------------------------------
| Results
|----------------------------------------------------------------------------*/

.lm-CommandPalette-header:first-child {
  margin-top: 0;
}

.lm-CommandPalette-header {
  border-bottom: solid var(--jp-border-width) var(--jp-border-color2);
  color: var(--jp-ui-font-color1);
  cursor: pointer;
  display: flex;
  font-size: var(--jp-ui-font-size0);
  font-weight: 600;
  letter-spacing: 1px;
  margin-top: 8px;
  padding: 8px 0 8px 12px;
  text-transform: uppercase;
}

.lm-CommandPalette-header.lm-mod-active {
  background: var(--jp-layout-color2);
}

.lm-CommandPalette-header > mark {
  background-color: transparent;
  font-weight: bold;
  color: var(--jp-ui-font-color1);
}

.lm-CommandPalette-item {
  padding: 4px 12px 4px 4px;
  color: var(--jp-ui-font-color1);
  font-size: var(--jp-ui-font-size1);
  font-weight: 400;
  display: flex;
}

.lm-CommandPalette-item.lm-mod-disabled {
  color: var(--jp-ui-font-color2);
}

.lm-CommandPalette-item.lm-mod-active {
  color: var(--jp-ui-inverse-font-color1);
  background: var(--jp-brand-color1);
}

.lm-CommandPalette-item.lm-mod-active .lm-CommandPalette-itemLabel > mark {
  color: var(--jp-ui-inverse-font-color0);
}

.lm-CommandPalette-item.lm-mod-active .jp-icon-selectable[fill] {
  fill: var(--jp-layout-color0);
}

.lm-CommandPalette-item.lm-mod-active:hover:not(.lm-mod-disabled) {
  color: var(--jp-ui-inverse-font-color1);
  background: var(--jp-brand-color1);
}

.lm-CommandPalette-item:hover:not(.lm-mod-active):not(.lm-mod-disabled) {
  background: var(--jp-layout-color2);
}

.lm-CommandPalette-itemContent {
  overflow: hidden;
}

.lm-CommandPalette-itemLabel > mark {
  color: var(--jp-ui-font-color0);
  background-color: transparent;
  font-weight: bold;
}

.lm-CommandPalette-item.lm-mod-disabled mark {
  color: var(--jp-ui-font-color2);
}

.lm-CommandPalette-item .lm-CommandPalette-itemIcon {
  margin: 0 4px 0 0;
  position: relative;
  width: 16px;
  top: 2px;
  flex: 0 0 auto;
}

.lm-CommandPalette-item.lm-mod-disabled .lm-CommandPalette-itemIcon {
  opacity: 0.6;
}

.lm-CommandPalette-item .lm-CommandPalette-itemShortcut {
  flex: 0 0 auto;
}

.lm-CommandPalette-itemCaption {
  display: none;
}

.lm-CommandPalette-content {
  background-color: var(--jp-layout-color1);
}

.lm-CommandPalette-content:empty::after {
  content: 'No results';
  margin: auto;
  margin-top: 20px;
  width: 100px;
  display: block;
  font-size: var(--jp-ui-font-size2);
  font-family: var(--jp-ui-font-family);
  font-weight: lighter;
}

.lm-CommandPalette-emptyMessage {
  text-align: center;
  margin-top: 24px;
  line-height: 1.32;
  padding: 0 8px;
  color: var(--jp-content-font-color3);
}

/*-----------------------------------------------------------------------------
| Copyright (c) 2014-2017, Jupyter Development Team.
|
| Distributed under the terms of the Modified BSD License.
|----------------------------------------------------------------------------*/

.jp-Dialog {
  position: absolute;
  z-index: 10000;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  top: 0;
  left: 0;
  margin: 0;
  padding: 0;
  width: 100%;
  height: 100%;
  background: var(--jp-dialog-background);
}

.jp-Dialog-content {
  display: flex;
  flex-direction: column;
  margin-left: auto;
  margin-right: auto;
  background: var(--jp-layout-color1);
  padding: 24px 24px 12px;
  min-width: 300px;
  min-height: 150px;
  max-width: 1000px;
  max-height: 500px;
  box-sizing: border-box;
  box-shadow: var(--jp-elevation-z20);
  word-wrap: break-word;
  border-radius: var(--jp-border-radius);

  /* This is needed so that all font sizing of children done in ems is
   * relative to this base size */
  font-size: var(--jp-ui-font-size1);
  color: var(--jp-ui-font-color1);
  resize: both;
}

.jp-Dialog-content.jp-Dialog-content-small {
  max-width: 500px;
}

.jp-Dialog-button {
  overflow: visible;
}

button.jp-Dialog-button:focus {
  outline: 1px solid var(--jp-brand-color1);
  outline-offset: 4px;
  -moz-outline-radius: 0;
}

button.jp-Dialog-button:focus::-moz-focus-inner {
  border: 0;
}

button.jp-Dialog-button.jp-mod-styled.jp-mod-accept:focus,
button.jp-Dialog-button.jp-mod-styled.jp-mod-warn:focus,
button.jp-Dialog-button.jp-mod-styled.jp-mod-reject:focus {
  outline-offset: 4px;
  -moz-outline-radius: 0;
}

button.jp-Dialog-button.jp-mod-styled.jp-mod-accept:focus {
  outline: 1px solid var(--jp-accept-color-normal, var(--jp-brand-color1));
}

button.jp-Dialog-button.jp-mod-styled.jp-mod-warn:focus {
  outline: 1px solid var(--jp-warn-color-normal, var(--jp-error-color1));
}

button.jp-Dialog-button.jp-mod-styled.jp-mod-reject:focus {
  outline: 1px solid var(--jp-reject-color-normal, var(--md-grey-600));
}

button.jp-Dialog-close-button {
  padding: 0;
  height: 100%;
  min-width: unset;
  min-height: unset;
}

.jp-Dialog-header {
  display: flex;
  justify-content: space-between;
  flex: 0 0 auto;
  padding-bottom: 12px;
  font-size: var(--jp-ui-font-size3);
  font-weight: 400;
  color: var(--jp-ui-font-color1);
}

.jp-Dialog-body {
  display: flex;
  flex-direction: column;
  flex: 1 1 auto;
  font-size: var(--jp-ui-font-size1);
  background: var(--jp-layout-color1);
  color: var(--jp-ui-font-color1);
  overflow: auto;
}

.jp-Dialog-footer {
  display: flex;
  flex-direction: row;
  justify-content: flex-end;
  align-items: center;
  flex: 0 0 auto;
  margin-left: -12px;
  margin-right: -12px;
  padding: 12px;
}

.jp-Dialog-checkbox {
  padding-right: 5px;
}

.jp-Dialog-checkbox > input:focus-visible {
  outline: 1px solid var(--jp-input-active-border-color);
  outline-offset: 1px;
}

.jp-Dialog-spacer {
  flex: 1 1 auto;
}

.jp-Dialog-title {
  overflow: hidden;
  white-space: nowrap;
  text-overflow: ellipsis;
}

.jp-Dialog-body > .jp-select-wrapper {
  width: 100%;
}

.jp-Dialog-body > button {
  padding: 0 16px;
}

.jp-Dialog-body > label {
  line-height: 1.4;
  color: var(--jp-ui-font-color0);
}

.jp-Dialog-button.jp-mod-styled:not(:last-child) {
  margin-right: 12px;
}

/*
 * Copyright (c) Jupyter Development Team.
 * Distributed under the terms of the Modified BSD License.
 */

.jp-Input-Boolean-Dialog {
  flex-direction: row-reverse;
  align-items: end;
  width: 100%;
}

.jp-Input-Boolean-Dialog > label {
  flex: 1 1 auto;
}

/*-----------------------------------------------------------------------------
| Copyright (c) 2014-2016, Jupyter Development Team.
|
| Distributed under the terms of the Modified BSD License.
|----------------------------------------------------------------------------*/

.jp-MainAreaWidget > :focus {
  outline: none;
}

.jp-MainAreaWidget .jp-MainAreaWidget-error {
  padding: 6px;
}

.jp-MainAreaWidget .jp-MainAreaWidget-error > pre {
  width: auto;
  padding: 10px;
  background: var(--jp-error-color3);
  border: var(--jp-border-width) solid var(--jp-error-color1);
  border-radius: var(--jp-border-radius);
  color: var(--jp-ui-font-color1);
  font-size: var(--jp-ui-font-size1);
  white-space: pre-wrap;
  word-wrap: break-word;
}

/*
 * Copyright (c) Jupyter Development Team.
 * Distributed under the terms of the Modified BSD License.
 */

/**
 * google-material-color v1.2.6
 * https://github.com/danlevan/google-material-color
 */
:root {
  --md-red-50: #ffebee;
  --md-red-100: #ffcdd2;
  --md-red-200: #ef9a9a;
  --md-red-300: #e57373;
  --md-red-400: #ef5350;
  --md-red-500: #f44336;
  --md-red-600: #e53935;
  --md-red-700: #d32f2f;
  --md-red-800: #c62828;
  --md-red-900: #b71c1c;
  --md-red-A100: #ff8a80;
  --md-red-A200: #ff5252;
  --md-red-A400: #ff1744;
  --md-red-A700: #d50000;
  --md-pink-50: #fce4ec;
  --md-pink-100: #f8bbd0;
  --md-pink-200: #f48fb1;
  --md-pink-300: #f06292;
  --md-pink-400: #ec407a;
  --md-pink-500: #e91e63;
  --md-pink-600: #d81b60;
  --md-pink-700: #c2185b;
  --md-pink-800: #ad1457;
  --md-pink-900: #880e4f;
  --md-pink-A100: #ff80ab;
  --md-pink-A200: #ff4081;
  --md-pink-A400: #f50057;
  --md-pink-A700: #c51162;
  --md-purple-50: #f3e5f5;
  --md-purple-100: #e1bee7;
  --md-purple-200: #ce93d8;
  --md-purple-300: #ba68c8;
  --md-purple-400: #ab47bc;
  --md-purple-500: #9c27b0;
  --md-purple-600: #8e24aa;
  --md-purple-700: #7b1fa2;
  --md-purple-800: #6a1b9a;
  --md-purple-900: #4a148c;
  --md-purple-A100: #ea80fc;
  --md-purple-A200: #e040fb;
  --md-purple-A400: #d500f9;
  --md-purple-A700: #a0f;
  --md-deep-purple-50: #ede7f6;
  --md-deep-purple-100: #d1c4e9;
  --md-deep-purple-200: #b39ddb;
  --md-deep-purple-300: #9575cd;
  --md-deep-purple-400: #7e57c2;
  --md-deep-purple-500: #673ab7;
  --md-deep-purple-600: #5e35b1;
  --md-deep-purple-700: #512da8;
  --md-deep-purple-800: #4527a0;
  --md-deep-purple-900: #311b92;
  --md-deep-purple-A100: #b388ff;
  --md-deep-purple-A200: #7c4dff;
  --md-deep-purple-A400: #651fff;
  --md-deep-purple-A700: #6200ea;
  --md-indigo-50: #e8eaf6;
  --md-indigo-100: #c5cae9;
  --md-indigo-200: #9fa8da;
  --md-indigo-300: #7986cb;
  --md-indigo-400: #5c6bc0;
  --md-indigo-500: #3f51b5;
  --md-indigo-600: #3949ab;
  --md-indigo-700: #303f9f;
  --md-indigo-800: #283593;
  --md-indigo-900: #1a237e;
  --md-indigo-A100: #8c9eff;
  --md-indigo-A200: #536dfe;
  --md-indigo-A400: #3d5afe;
  --md-indigo-A700: #304ffe;
  --md-blue-50: #e3f2fd;
  --md-blue-100: #bbdefb;
  --md-blue-200: #90caf9;
  --md-blue-300: #64b5f6;
  --md-blue-400: #42a5f5;
  --md-blue-500: #2196f3;
  --md-blue-600: #1e88e5;
  --md-blue-700: #1976d2;
  --md-blue-800: #1565c0;
  --md-blue-900: #0d47a1;
  --md-blue-A100: #82b1ff;
  --md-blue-A200: #448aff;
  --md-blue-A400: #2979ff;
  --md-blue-A700: #2962ff;
  --md-light-blue-50: #e1f5fe;
  --md-light-blue-100: #b3e5fc;
  --md-light-blue-200: #81d4fa;
  --md-light-blue-300: #4fc3f7;
  --md-light-blue-400: #29b6f6;
  --md-light-blue-500: #03a9f4;
  --md-light-blue-600: #039be5;
  --md-light-blue-700: #0288d1;
  --md-light-blue-800: #0277bd;
  --md-light-blue-900: #01579b;
  --md-light-blue-A100: #80d8ff;
  --md-light-blue-A200: #40c4ff;
  --md-light-blue-A400: #00b0ff;
  --md-light-blue-A700: #0091ea;
  --md-cyan-50: #e0f7fa;
  --md-cyan-100: #b2ebf2;
  --md-cyan-200: #80deea;
  --md-cyan-300: #4dd0e1;
  --md-cyan-400: #26c6da;
  --md-cyan-500: #00bcd4;
  --md-cyan-600: #00acc1;
  --md-cyan-700: #0097a7;
  --md-cyan-800: #00838f;
  --md-cyan-900: #006064;
  --md-cyan-A100: #84ffff;
  --md-cyan-A200: #18ffff;
  --md-cyan-A400: #00e5ff;
  --md-cyan-A700: #00b8d4;
  --md-teal-50: #e0f2f1;
  --md-teal-100: #b2dfdb;
  --md-teal-200: #80cbc4;
  --md-teal-300: #4db6ac;
  --md-teal-400: #26a69a;
  --md-teal-500: #009688;
  --md-teal-600: #00897b;
  --md-teal-700: #00796b;
  --md-teal-800: #00695c;
  --md-teal-900: #004d40;
  --md-teal-A100: #a7ffeb;
  --md-teal-A200: #64ffda;
  --md-teal-A400: #1de9b6;
  --md-teal-A700: #00bfa5;
  --md-green-50: #e8f5e9;
  --md-green-100: #c8e6c9;
  --md-green-200: #a5d6a7;
  --md-green-300: #81c784;
  --md-green-400: #66bb6a;
  --md-green-500: #4caf50;
  --md-green-600: #43a047;
  --md-green-700: #388e3c;
  --md-green-800: #2e7d32;
  --md-green-900: #1b5e20;
  --md-green-A100: #b9f6ca;
  --md-green-A200: #69f0ae;
  --md-green-A400: #00e676;
  --md-green-A700: #00c853;
  --md-light-green-50: #f1f8e9;
  --md-light-green-100: #dcedc8;
  --md-light-green-200: #c5e1a5;
  --md-light-green-300: #aed581;
  --md-light-green-400: #9ccc65;
  --md-light-green-500: #8bc34a;
  --md-light-green-600: #7cb342;
  --md-light-green-700: #689f38;
  --md-light-green-800: #558b2f;
  --md-light-green-900: #33691e;
  --md-light-green-A100: #ccff90;
  --md-light-green-A200: #b2ff59;
  --md-light-green-A400: #76ff03;
  --md-light-green-A700: #64dd17;
  --md-lime-50: #f9fbe7;
  --md-lime-100: #f0f4c3;
  --md-lime-200: #e6ee9c;
  --md-lime-300: #dce775;
  --md-lime-400: #d4e157;
  --md-lime-500: #cddc39;
  --md-lime-600: #c0ca33;
  --md-lime-700: #afb42b;
  --md-lime-800: #9e9d24;
  --md-lime-900: #827717;
  --md-lime-A100: #f4ff81;
  --md-lime-A200: #eeff41;
  --md-lime-A400: #c6ff00;
  --md-lime-A700: #aeea00;
  --md-yellow-50: #fffde7;
  --md-yellow-100: #fff9c4;
  --md-yellow-200: #fff59d;
  --md-yellow-300: #fff176;
  --md-yellow-400: #ffee58;
  --md-yellow-500: #ffeb3b;
  --md-yellow-600: #fdd835;
  --md-yellow-700: #fbc02d;
  --md-yellow-800: #f9a825;
  --md-yellow-900: #f57f17;
  --md-yellow-A100: #ffff8d;
  --md-yellow-A200: #ff0;
  --md-yellow-A400: #ffea00;
  --md-yellow-A700: #ffd600;
  --md-amber-50: #fff8e1;
  --md-amber-100: #ffecb3;
  --md-amber-200: #ffe082;
  --md-amber-300: #ffd54f;
  --md-amber-400: #ffca28;
  --md-amber-500: #ffc107;
  --md-amber-600: #ffb300;
  --md-amber-700: #ffa000;
  --md-amber-800: #ff8f00;
  --md-amber-900: #ff6f00;
  --md-amber-A100: #ffe57f;
  --md-amber-A200: #ffd740;
  --md-amber-A400: #ffc400;
  --md-amber-A700: #ffab00;
  --md-orange-50: #fff3e0;
  --md-orange-100: #ffe0b2;
  --md-orange-200: #ffcc80;
  --md-orange-300: #ffb74d;
  --md-orange-400: #ffa726;
  --md-orange-500: #ff9800;
  --md-orange-600: #fb8c00;
  --md-orange-700: #f57c00;
  --md-orange-800: #ef6c00;
  --md-orange-900: #e65100;
  --md-orange-A100: #ffd180;
  --md-orange-A200: #ffab40;
  --md-orange-A400: #ff9100;
  --md-orange-A700: #ff6d00;
  --md-deep-orange-50: #fbe9e7;
  --md-deep-orange-100: #ffccbc;
  --md-deep-orange-200: #ffab91;
  --md-deep-orange-300: #ff8a65;
  --md-deep-orange-400: #ff7043;
  --md-deep-orange-500: #ff5722;
  --md-deep-orange-600: #f4511e;
  --md-deep-orange-700: #e64a19;
  --md-deep-orange-800: #d84315;
  --md-deep-orange-900: #bf360c;
  --md-deep-orange-A100: #ff9e80;
  --md-deep-orange-A200: #ff6e40;
  --md-deep-orange-A400: #ff3d00;
  --md-deep-orange-A700: #dd2c00;
  --md-brown-50: #efebe9;
  --md-brown-100: #d7ccc8;
  --md-brown-200: #bcaaa4;
  --md-brown-300: #a1887f;
  --md-brown-400: #8d6e63;
  --md-brown-500: #795548;
  --md-brown-600: #6d4c41;
  --md-brown-700: #5d4037;
  --md-brown-800: #4e342e;
  --md-brown-900: #3e2723;
  --md-grey-50: #fafafa;
  --md-grey-100: #f5f5f5;
  --md-grey-200: #eee;
  --md-grey-300: #e0e0e0;
  --md-grey-400: #bdbdbd;
  --md-grey-500: #9e9e9e;
  --md-grey-600: #757575;
  --md-grey-700: #616161;
  --md-grey-800: #424242;
  --md-grey-900: #212121;
  --md-blue-grey-50: #eceff1;
  --md-blue-grey-100: #cfd8dc;
  --md-blue-grey-200: #b0bec5;
  --md-blue-grey-300: #90a4ae;
  --md-blue-grey-400: #78909c;
  --md-blue-grey-500: #607d8b;
  --md-blue-grey-600: #546e7a;
  --md-blue-grey-700: #455a64;
  --md-blue-grey-800: #37474f;
  --md-blue-grey-900: #263238;
}

/*-----------------------------------------------------------------------------
| Copyright (c) 2014-2017, Jupyter Development Team.
|
| Distributed under the terms of the Modified BSD License.
|----------------------------------------------------------------------------*/

/*-----------------------------------------------------------------------------
| Copyright (c) Jupyter Development Team.
| Distributed under the terms of the Modified BSD License.
|----------------------------------------------------------------------------*/

/*-----------------------------------------------------------------------------
| RenderedText
|----------------------------------------------------------------------------*/

:root {
  /* This is the padding value to fill the gaps between lines containing spans with background color. */
  --jp-private-code-span-padding: calc(
    (var(--jp-code-line-height) - 1) * var(--jp-code-font-size) / 2
  );
}

.jp-RenderedText {
  text-align: left;
  padding-left: var(--jp-code-padding);
  line-height: var(--jp-code-line-height);
  font-family: var(--jp-code-font-family);
}

.jp-RenderedText pre,
.jp-RenderedJavaScript pre,
.jp-RenderedHTMLCommon pre {
  color: var(--jp-content-font-color1);
  font-size: var(--jp-code-font-size);
  border: none;
  margin: 0;
  padding: 0;
}

.jp-RenderedText pre a:link {
  text-decoration: none;
  color: var(--jp-content-link-color);
}

.jp-RenderedText pre a:hover {
  text-decoration: underline;
  color: var(--jp-content-link-color);
}

.jp-RenderedText pre a:visited {
  text-decoration: none;
  color: var(--jp-content-link-color);
}

/* console foregrounds and backgrounds */
.jp-RenderedText pre .ansi-black-fg {
  color: #3e424d;
}

.jp-RenderedText pre .ansi-red-fg {
  color: #e75c58;
}

.jp-RenderedText pre .ansi-green-fg {
  color: #00a250;
}

.jp-RenderedText pre .ansi-yellow-fg {
  color: #ddb62b;
}

.jp-RenderedText pre .ansi-blue-fg {
  color: #208ffb;
}

.jp-RenderedText pre .ansi-magenta-fg {
  color: #d160c4;
}

.jp-RenderedText pre .ansi-cyan-fg {
  color: #60c6c8;
}

.jp-RenderedText pre .ansi-white-fg {
  color: #c5c1b4;
}

.jp-RenderedText pre .ansi-black-bg {
  background-color: #3e424d;
  padding: var(--jp-private-code-span-padding) 0;
}

.jp-RenderedText pre .ansi-red-bg {
  background-color: #e75c58;
  padding: var(--jp-private-code-span-padding) 0;
}

.jp-RenderedText pre .ansi-green-bg {
  background-color: #00a250;
  padding: var(--jp-private-code-span-padding) 0;
}

.jp-RenderedText pre .ansi-yellow-bg {
  background-color: #ddb62b;
  padding: var(--jp-private-code-span-padding) 0;
}

.jp-RenderedText pre .ansi-blue-bg {
  background-color: #208ffb;
  padding: var(--jp-private-code-span-padding) 0;
}

.jp-RenderedText pre .ansi-magenta-bg {
  background-color: #d160c4;
  padding: var(--jp-private-code-span-padding) 0;
}

.jp-RenderedText pre .ansi-cyan-bg {
  background-color: #60c6c8;
  padding: var(--jp-private-code-span-padding) 0;
}

.jp-RenderedText pre .ansi-white-bg {
  background-color: #c5c1b4;
  padding: var(--jp-private-code-span-padding) 0;
}

.jp-RenderedText pre .ansi-black-intense-fg {
  color: #282c36;
}

.jp-RenderedText pre .ansi-red-intense-fg {
  color: #b22b31;
}

.jp-RenderedText pre .ansi-green-intense-fg {
  color: #007427;
}

.jp-RenderedText pre .ansi-yellow-intense-fg {
  color: #b27d12;
}

.jp-RenderedText pre .ansi-blue-intense-fg {
  color: #0065ca;
}

.jp-RenderedText pre .ansi-magenta-intense-fg {
  color: #a03196;
}

.jp-RenderedText pre .ansi-cyan-intense-fg {
  color: #258f8f;
}

.jp-RenderedText pre .ansi-white-intense-fg {
  color: #a1a6b2;
}

.jp-RenderedText pre .ansi-black-intense-bg {
  background-color: #282c36;
  padding: var(--jp-private-code-span-padding) 0;
}

.jp-RenderedText pre .ansi-red-intense-bg {
  background-color: #b22b31;
  padding: var(--jp-private-code-span-padding) 0;
}

.jp-RenderedText pre .ansi-green-intense-bg {
  background-color: #007427;
  padding: var(--jp-private-code-span-padding) 0;
}

.jp-RenderedText pre .ansi-yellow-intense-bg {
  background-color: #b27d12;
  padding: var(--jp-private-code-span-padding) 0;
}

.jp-RenderedText pre .ansi-blue-intense-bg {
  background-color: #0065ca;
  padding: var(--jp-private-code-span-padding) 0;
}

.jp-RenderedText pre .ansi-magenta-intense-bg {
  background-color: #a03196;
  padding: var(--jp-private-code-span-padding) 0;
}

.jp-RenderedText pre .ansi-cyan-intense-bg {
  background-color: #258f8f;
  padding: var(--jp-private-code-span-padding) 0;
}

.jp-RenderedText pre .ansi-white-intense-bg {
  background-color: #a1a6b2;
  padding: var(--jp-private-code-span-padding) 0;
}

.jp-RenderedText pre .ansi-default-inverse-fg {
  color: var(--jp-ui-inverse-font-color0);
}

.jp-RenderedText pre .ansi-default-inverse-bg {
  background-color: var(--jp-inverse-layout-color0);
  padding: var(--jp-private-code-span-padding) 0;
}

.jp-RenderedText pre .ansi-bold {
  font-weight: bold;
}

.jp-RenderedText pre .ansi-underline {
  text-decoration: underline;
}

.jp-RenderedText[data-mime-type='application/vnd.jupyter.stderr'] {
  background: var(--jp-rendermime-error-background);
  padding-top: var(--jp-code-padding);
}

/*-----------------------------------------------------------------------------
| RenderedLatex
|----------------------------------------------------------------------------*/

.jp-RenderedLatex {
  color: var(--jp-content-font-color1);
  font-size: var(--jp-content-font-size1);
  line-height: var(--jp-content-line-height);
}

/* Left-justify outputs.*/
.jp-OutputArea-output.jp-RenderedLatex {
  padding: var(--jp-code-padding);
  text-align: left;
}

/*-----------------------------------------------------------------------------
| RenderedHTML
|----------------------------------------------------------------------------*/

.jp-RenderedHTMLCommon {
  color: var(--jp-content-font-color1);
  font-family: var(--jp-content-font-family);
  font-size: var(--jp-content-font-size1);
  line-height: var(--jp-content-line-height);

  /* Give a bit more R padding on Markdown text to keep line lengths reasonable */
  padding-right: 20px;
}

.jp-RenderedHTMLCommon em {
  font-style: italic;
}

.jp-RenderedHTMLCommon strong {
  font-weight: bold;
}

.jp-RenderedHTMLCommon u {
  text-decoration: underline;
}

.jp-RenderedHTMLCommon a:link {
  text-decoration: none;
  color: var(--jp-content-link-color);
}

.jp-RenderedHTMLCommon a:hover {
  text-decoration: underline;
  color: var(--jp-content-link-color);
}

.jp-RenderedHTMLCommon a:visited {
  text-decoration: none;
  color: var(--jp-content-link-color);
}

/* Headings */

.jp-RenderedHTMLCommon h1,
.jp-RenderedHTMLCommon h2,
.jp-RenderedHTMLCommon h3,
.jp-RenderedHTMLCommon h4,
.jp-RenderedHTMLCommon h5,
.jp-RenderedHTMLCommon h6 {
  line-height: var(--jp-content-heading-line-height);
  font-weight: var(--jp-content-heading-font-weight);
  font-style: normal;
  margin: var(--jp-content-heading-margin-top) 0
    var(--jp-content-heading-margin-bottom) 0;
}

.jp-RenderedHTMLCommon h1:first-child,
.jp-RenderedHTMLCommon h2:first-child,
.jp-RenderedHTMLCommon h3:first-child,
.jp-RenderedHTMLCommon h4:first-child,
.jp-RenderedHTMLCommon h5:first-child,
.jp-RenderedHTMLCommon h6:first-child {
  margin-top: calc(0.5 * var(--jp-content-heading-margin-top));
}

.jp-RenderedHTMLCommon h1:last-child,
.jp-RenderedHTMLCommon h2:last-child,
.jp-RenderedHTMLCommon h3:last-child,
.jp-RenderedHTMLCommon h4:last-child,
.jp-RenderedHTMLCommon h5:last-child,
.jp-RenderedHTMLCommon h6:last-child {
  margin-bottom: calc(0.5 * var(--jp-content-heading-margin-bottom));
}

.jp-RenderedHTMLCommon h1 {
  font-size: var(--jp-content-font-size5);
}

.jp-RenderedHTMLCommon h2 {
  font-size: var(--jp-content-font-size4);
}

.jp-RenderedHTMLCommon h3 {
  font-size: var(--jp-content-font-size3);
}

.jp-RenderedHTMLCommon h4 {
  font-size: var(--jp-content-font-size2);
}

.jp-RenderedHTMLCommon h5 {
  font-size: var(--jp-content-font-size1);
}

.jp-RenderedHTMLCommon h6 {
  font-size: var(--jp-content-font-size0);
}

/* Lists */

/* stylelint-disable selector-max-type, selector-max-compound-selectors */

.jp-RenderedHTMLCommon ul:not(.list-inline),
.jp-RenderedHTMLCommon ol:not(.list-inline) {
  padding-left: 2em;
}

.jp-RenderedHTMLCommon ul {
  list-style: disc;
}

.jp-RenderedHTMLCommon ul ul {
  list-style: square;
}

.jp-RenderedHTMLCommon ul ul ul {
  list-style: circle;
}

.jp-RenderedHTMLCommon ol {
  list-style: decimal;
}

.jp-RenderedHTMLCommon ol ol {
  list-style: upper-alpha;
}

.jp-RenderedHTMLCommon ol ol ol {
  list-style: lower-alpha;
}

.jp-RenderedHTMLCommon ol ol ol ol {
  list-style: lower-roman;
}

.jp-RenderedHTMLCommon ol ol ol ol ol {
  list-style: decimal;
}

.jp-RenderedHTMLCommon ol,
.jp-RenderedHTMLCommon ul {
  margin-bottom: 1em;
}

.jp-RenderedHTMLCommon ul ul,
.jp-RenderedHTMLCommon ul ol,
.jp-RenderedHTMLCommon ol ul,
.jp-RenderedHTMLCommon ol ol {
  margin-bottom: 0;
}

/* stylelint-enable selector-max-type, selector-max-compound-selectors */

.jp-RenderedHTMLCommon hr {
  color: var(--jp-border-color2);
  background-color: var(--jp-border-color1);
  margin-top: 1em;
  margin-bottom: 1em;
}

.jp-RenderedHTMLCommon > pre {
  margin: 1.5em 2em;
}

.jp-RenderedHTMLCommon pre,
.jp-RenderedHTMLCommon code {
  border: 0;
  background-color: var(--jp-layout-color0);
  color: var(--jp-content-font-color1);
  font-family: var(--jp-code-font-family);
  font-size: inherit;
  line-height: var(--jp-code-line-height);
  padding: 0;
  white-space: pre-wrap;
}

.jp-RenderedHTMLCommon :not(pre) > code {
  background-color: var(--jp-layout-color2);
  padding: 1px 5px;
}

/* Tables */

.jp-RenderedHTMLCommon table {
  border-collapse: collapse;
  border-spacing: 0;
  border: none;
  color: var(--jp-ui-font-color1);
  font-size: var(--jp-ui-font-size1);
  table-layout: fixed;
  margin-left: auto;
  margin-bottom: 1em;
  margin-right: auto;
}

.jp-RenderedHTMLCommon thead {
  border-bottom: var(--jp-border-width) solid var(--jp-border-color1);
  vertical-align: bottom;
}

.jp-RenderedHTMLCommon td,
.jp-RenderedHTMLCommon th,
.jp-RenderedHTMLCommon tr {
  vertical-align: middle;
  padding: 0.5em;
  line-height: normal;
  white-space: normal;
  max-width: none;
  border: none;
}

.jp-RenderedMarkdown.jp-RenderedHTMLCommon td,
.jp-RenderedMarkdown.jp-RenderedHTMLCommon th {
  max-width: none;
}

:not(.jp-RenderedMarkdown).jp-RenderedHTMLCommon td,
:not(.jp-RenderedMarkdown).jp-RenderedHTMLCommon th,
:not(.jp-RenderedMarkdown).jp-RenderedHTMLCommon tr {
  text-align: right;
}

.jp-RenderedHTMLCommon th {
  font-weight: bold;
}

.jp-RenderedHTMLCommon tbody tr:nth-child(odd) {
  background: var(--jp-layout-color0);
}

.jp-RenderedHTMLCommon tbody tr:nth-child(even) {
  background: var(--jp-rendermime-table-row-background);
}

.jp-RenderedHTMLCommon tbody tr:hover {
  background: var(--jp-rendermime-table-row-hover-background);
}

.jp-RenderedHTMLCommon p {
  text-align: left;
  margin: 0;
  margin-bottom: 1em;
}

.jp-RenderedHTMLCommon img {
  -moz-force-broken-image-icon: 1;
}

/* Restrict to direct children as other images could be nested in other content. */
.jp-RenderedHTMLCommon > img {
  display: block;
  margin-left: 0;
  margin-right: 0;
  margin-bottom: 1em;
}

/* Change color behind transparent images if they need it... */
[data-jp-theme-light='false'] .jp-RenderedImage img.jp-needs-light-background {
  background-color: var(--jp-inverse-layout-color1);
}

[data-jp-theme-light='true'] .jp-RenderedImage img.jp-needs-dark-background {
  background-color: var(--jp-inverse-layout-color1);
}

.jp-RenderedHTMLCommon img,
.jp-RenderedImage img,
.jp-RenderedHTMLCommon svg,
.jp-RenderedSVG svg {
  max-width: 100%;
  height: auto;
}

.jp-RenderedHTMLCommon img.jp-mod-unconfined,
.jp-RenderedImage img.jp-mod-unconfined,
.jp-RenderedHTMLCommon svg.jp-mod-unconfined,
.jp-RenderedSVG svg.jp-mod-unconfined {
  max-width: none;
}

.jp-RenderedHTMLCommon .alert {
  padding: var(--jp-notebook-padding);
  border: var(--jp-border-width) solid transparent;
  border-radius: var(--jp-border-radius);
  margin-bottom: 1em;
}

.jp-RenderedHTMLCommon .alert-info {
  color: var(--jp-info-color0);
  background-color: var(--jp-info-color3);
  border-color: var(--jp-info-color2);
}

.jp-RenderedHTMLCommon .alert-info hr {
  border-color: var(--jp-info-color3);
}

.jp-RenderedHTMLCommon .alert-info > p:last-child,
.jp-RenderedHTMLCommon .alert-info > ul:last-child {
  margin-bottom: 0;
}

.jp-RenderedHTMLCommon .alert-warning {
  color: var(--jp-warn-color0);
  background-color: var(--jp-warn-color3);
  border-color: var(--jp-warn-color2);
}

.jp-RenderedHTMLCommon .alert-warning hr {
  border-color: var(--jp-warn-color3);
}

.jp-RenderedHTMLCommon .alert-warning > p:last-child,
.jp-RenderedHTMLCommon .alert-warning > ul:last-child {
  margin-bottom: 0;
}

.jp-RenderedHTMLCommon .alert-success {
  color: var(--jp-success-color0);
  background-color: var(--jp-success-color3);
  border-color: var(--jp-success-color2);
}

.jp-RenderedHTMLCommon .alert-success hr {
  border-color: var(--jp-success-color3);
}

.jp-RenderedHTMLCommon .alert-success > p:last-child,
.jp-RenderedHTMLCommon .alert-success > ul:last-child {
  margin-bottom: 0;
}

.jp-RenderedHTMLCommon .alert-danger {
  color: var(--jp-error-color0);
  background-color: var(--jp-error-color3);
  border-color: var(--jp-error-color2);
}

.jp-RenderedHTMLCommon .alert-danger hr {
  border-color: var(--jp-error-color3);
}

.jp-RenderedHTMLCommon .alert-danger > p:last-child,
.jp-RenderedHTMLCommon .alert-danger > ul:last-child {
  margin-bottom: 0;
}

.jp-RenderedHTMLCommon blockquote {
  margin: 1em 2em;
  padding: 0 1em;
  border-left: 5px solid var(--jp-border-color2);
}

a.jp-InternalAnchorLink {
  visibility: hidden;
  margin-left: 8px;
  color: var(--md-blue-800);
}

h1:hover .jp-InternalAnchorLink,
h2:hover .jp-InternalAnchorLink,
h3:hover .jp-InternalAnchorLink,
h4:hover .jp-InternalAnchorLink,
h5:hover .jp-InternalAnchorLink,
h6:hover .jp-InternalAnchorLink {
  visibility: visible;
}

.jp-RenderedHTMLCommon kbd {
  background-color: var(--jp-rendermime-table-row-background);
  border: 1px solid var(--jp-border-color0);
  border-bottom-color: var(--jp-border-color2);
  border-radius: 3px;
  box-shadow: inset 0 -1px 0 rgba(0, 0, 0, 0.25);
  display: inline-block;
  font-size: var(--jp-ui-font-size0);
  line-height: 1em;
  padding: 0.2em 0.5em;
}

/* Most direct children of .jp-RenderedHTMLCommon have a margin-bottom of 1.0.
 * At the bottom of cells this is a bit too much as there is also spacing
 * between cells. Going all the way to 0 gets too tight between markdown and
 * code cells.
 */
.jp-RenderedHTMLCommon > *:last-child {
  margin-bottom: 0.5em;
}

/*
 * Copyright (c) Jupyter Development Team.
 * Distributed under the terms of the Modified BSD License.
 */

/*-----------------------------------------------------------------------------
| Copyright (c) Jupyter Development Team.
| Copyright (c) 2014-2017, PhosphorJS Contributors
|
| Distributed under the terms of the BSD 3-Clause License.
|
| The full license is in the file LICENSE, distributed with this software.
|----------------------------------------------------------------------------*/

.lm-cursor-backdrop {
  position: fixed;
  width: 200px;
  height: 200px;
  margin-top: -100px;
  margin-left: -100px;
  will-change: transform;
  z-index: 100;
}

.lm-mod-drag-image {
  will-change: transform;
}

/*
 * Copyright (c) Jupyter Development Team.
 * Distributed under the terms of the Modified BSD License.
 */

.jp-lineFormSearch {
  padding: 4px 12px;
  background-color: var(--jp-layout-color2);
  box-shadow: var(--jp-toolbar-box-shadow);
  z-index: 2;
  font-size: var(--jp-ui-font-size1);
}

.jp-lineFormCaption {
  font-size: var(--jp-ui-font-size0);
  line-height: var(--jp-ui-font-size1);
  margin-top: 4px;
  color: var(--jp-ui-font-color0);
}

.jp-baseLineForm {
  border: none;
  border-radius: 0;
  position: absolute;
  background-size: 16px;
  background-repeat: no-repeat;
  background-position: center;
  outline: none;
}

.jp-lineFormButtonContainer {
  top: 4px;
  right: 8px;
  height: 24px;
  padding: 0 12px;
  width: 12px;
}

.jp-lineFormButtonIcon {
  top: 0;
  right: 0;
  background-color: var(--jp-brand-color1);
  height: 100%;
  width: 100%;
  box-sizing: border-box;
  padding: 4px 6px;
}

.jp-lineFormButton {
  top: 0;
  right: 0;
  background-color: transparent;
  height: 100%;
  width: 100%;
  box-sizing: border-box;
}

.jp-lineFormWrapper {
  overflow: hidden;
  padding: 0 8px;
  border: 1px solid var(--jp-border-color0);
  background-color: var(--jp-input-active-background);
  height: 22px;
}

.jp-lineFormWrapperFocusWithin {
  border: var(--jp-border-width) solid var(--md-blue-500);
  box-shadow: inset 0 0 4px var(--md-blue-300);
}

.jp-lineFormInput {
  background: transparent;
  width: 200px;
  height: 100%;
  border: none;
  outline: none;
  color: var(--jp-ui-font-color0);
  line-height: 28px;
}

/*-----------------------------------------------------------------------------
| Copyright (c) 2014-2016, Jupyter Development Team.
|
| Distributed under the terms of the Modified BSD License.
|----------------------------------------------------------------------------*/

.jp-JSONEditor {
  display: flex;
  flex-direction: column;
  width: 100%;
}

.jp-JSONEditor-host {
  flex: 1 1 auto;
  border: var(--jp-border-width) solid var(--jp-input-border-color);
  border-radius: 0;
  background: var(--jp-layout-color0);
  min-height: 50px;
  padding: 1px;
}

.jp-JSONEditor.jp-mod-error .jp-JSONEditor-host {
  border-color: red;
  outline-color: red;
}

.jp-JSONEditor-header {
  display: flex;
  flex: 1 0 auto;
  padding: 0 0 0 12px;
}

.jp-JSONEditor-header label {
  flex: 0 0 auto;
}

.jp-JSONEditor-commitButton {
  height: 16px;
  width: 16px;
  background-size: 18px;
  background-repeat: no-repeat;
  background-position: center;
}

.jp-JSONEditor-host.jp-mod-focused {
  background-color: var(--jp-input-active-background);
  border: 1px solid var(--jp-input-active-border-color);
  box-shadow: var(--jp-input-box-shadow);
}

.jp-Editor.jp-mod-dropTarget {
  border: var(--jp-border-width) solid var(--jp-input-active-border-color);
  box-shadow: var(--jp-input-box-shadow);
}

/*-----------------------------------------------------------------------------
| Copyright (c) Jupyter Development Team.
| Distributed under the terms of the Modified BSD License.
|----------------------------------------------------------------------------*/
.jp-DocumentSearch-input {
  border: none;
  outline: none;
  color: var(--jp-ui-font-color0);
  font-size: var(--jp-ui-font-size1);
  background-color: var(--jp-layout-color0);
  font-family: var(--jp-ui-font-family);
  padding: 2px 1px;
  resize: none;
}

.jp-DocumentSearch-overlay {
  position: absolute;
  background-color: var(--jp-toolbar-background);
  border-bottom: var(--jp-border-width) solid var(--jp-toolbar-border-color);
  border-left: var(--jp-border-width) solid var(--jp-toolbar-border-color);
  top: 0;
  right: 0;
  z-index: 7;
  min-width: 405px;
  padding: 2px;
  font-size: var(--jp-ui-font-size1);

  --jp-private-document-search-button-height: 20px;
}

.jp-DocumentSearch-overlay button {
  background-color: var(--jp-toolbar-background);
  outline: 0;
}

.jp-DocumentSearch-overlay button:hover {
  background-color: var(--jp-layout-color2);
}

.jp-DocumentSearch-overlay button:active {
  background-color: var(--jp-layout-color3);
}

.jp-DocumentSearch-overlay-row {
  display: flex;
  align-items: center;
  margin-bottom: 2px;
}

.jp-DocumentSearch-button-content {
  display: inline-block;
  cursor: pointer;
  box-sizing: border-box;
  width: 100%;
  height: 100%;
}

.jp-DocumentSearch-button-content svg {
  width: 100%;
  height: 100%;
}

.jp-DocumentSearch-input-wrapper {
  border: var(--jp-border-width) solid var(--jp-border-color0);
  display: flex;
  background-color: var(--jp-layout-color0);
  margin: 2px;
}

.jp-DocumentSearch-input-wrapper:focus-within {
  border-color: var(--jp-cell-editor-active-border-color);
}

.jp-DocumentSearch-toggle-wrapper,
.jp-DocumentSearch-button-wrapper {
  all: initial;
  overflow: hidden;
  display: inline-block;
  border: none;
  box-sizing: border-box;
}

.jp-DocumentSearch-toggle-wrapper {
  width: 14px;
  height: 14px;
}

.jp-DocumentSearch-button-wrapper {
  width: var(--jp-private-document-search-button-height);
  height: var(--jp-private-document-search-button-height);
}

.jp-DocumentSearch-toggle-wrapper:focus,
.jp-DocumentSearch-button-wrapper:focus {
  outline: var(--jp-border-width) solid
    var(--jp-cell-editor-active-border-color);
  outline-offset: -1px;
}

.jp-DocumentSearch-toggle-wrapper,
.jp-DocumentSearch-button-wrapper,
.jp-DocumentSearch-button-content:focus {
  outline: none;
}

.jp-DocumentSearch-toggle-placeholder {
  width: 5px;
}

.jp-DocumentSearch-input-button::before {
  display: block;
  padding-top: 100%;
}

.jp-DocumentSearch-input-button-off {
  opacity: var(--jp-search-toggle-off-opacity);
}

.jp-DocumentSearch-input-button-off:hover {
  opacity: var(--jp-search-toggle-hover-opacity);
}

.jp-DocumentSearch-input-button-on {
  opacity: var(--jp-search-toggle-on-opacity);
}

.jp-DocumentSearch-index-counter {
  padding-left: 10px;
  padding-right: 10px;
  user-select: none;
  min-width: 35px;
  display: inline-block;
}

.jp-DocumentSearch-up-down-wrapper {
  display: inline-block;
  padding-right: 2px;
  margin-left: auto;
  white-space: nowrap;
}

.jp-DocumentSearch-spacer {
  margin-left: auto;
}

.jp-DocumentSearch-up-down-wrapper button {
  outline: 0;
  border: none;
  width: var(--jp-private-document-search-button-height);
  height: var(--jp-private-document-search-button-height);
  vertical-align: middle;
  margin: 1px 5px 2px;
}

.jp-DocumentSearch-up-down-button:hover {
  background-color: var(--jp-layout-color2);
}

.jp-DocumentSearch-up-down-button:active {
  background-color: var(--jp-layout-color3);
}

.jp-DocumentSearch-filter-button {
  border-radius: var(--jp-border-radius);
}

.jp-DocumentSearch-filter-button:hover {
  background-color: var(--jp-layout-color2);
}

.jp-DocumentSearch-filter-button-enabled {
  background-color: var(--jp-layout-color2);
}

.jp-DocumentSearch-filter-button-enabled:hover {
  background-color: var(--jp-layout-color3);
}

.jp-DocumentSearch-search-options {
  padding: 0 8px;
  margin-left: 3px;
  width: 100%;
  display: grid;
  justify-content: start;
  grid-template-columns: 1fr 1fr;
  align-items: center;
  justify-items: stretch;
}

.jp-DocumentSearch-search-filter-disabled {
  color: var(--jp-ui-font-color2);
}

.jp-DocumentSearch-search-filter {
  display: flex;
  align-items: center;
  user-select: none;
}

.jp-DocumentSearch-regex-error {
  color: var(--jp-error-color0);
}

.jp-DocumentSearch-replace-button-wrapper {
  overflow: hidden;
  display: inline-block;
  box-sizing: border-box;
  border: var(--jp-border-width) solid var(--jp-border-color0);
  margin: auto 2px;
  padding: 1px 4px;
  height: calc(var(--jp-private-document-search-button-height) + 2px);
}

.jp-DocumentSearch-replace-button-wrapper:focus {
  border: var(--jp-border-width) solid var(--jp-cell-editor-active-border-color);
}

.jp-DocumentSearch-replace-button {
  display: inline-block;
  text-align: center;
  cursor: pointer;
  box-sizing: border-box;
  color: var(--jp-ui-font-color1);

  /* height - 2 * (padding of wrapper) */
  line-height: calc(var(--jp-private-document-search-button-height) - 2px);
  width: 100%;
  height: 100%;
}

.jp-DocumentSearch-replace-button:focus {
  outline: none;
}

.jp-DocumentSearch-replace-wrapper-class {
  margin-left: 14px;
  display: flex;
}

.jp-DocumentSearch-replace-toggle {
  border: none;
  background-color: var(--jp-toolbar-background);
  border-radius: var(--jp-border-radius);
}

.jp-DocumentSearch-replace-toggle:hover {
  background-color: var(--jp-layout-color2);
}

/*-----------------------------------------------------------------------------
| Copyright (c) Jupyter Development Team.
| Distributed under the terms of the Modified BSD License.
|----------------------------------------------------------------------------*/

.cm-editor {
  line-height: var(--jp-code-line-height);
  font-size: var(--jp-code-font-size);
  font-family: var(--jp-code-font-family);
  border: 0;
  border-radius: 0;
  height: auto;

  /* Changed to auto to autogrow */
}

.cm-editor pre {
  padding: 0 var(--jp-code-padding);
}

.jp-CodeMirrorEditor[data-type='inline'] .cm-dialog {
  background-color: var(--jp-layout-color0);
  color: var(--jp-content-font-color1);
}

.jp-CodeMirrorEditor {
  cursor: text;
}

/* When zoomed out 67% and 33% on a screen of 1440 width x 900 height */
@media screen and (min-width: 2138px) and (max-width: 4319px) {
  .jp-CodeMirrorEditor[data-type='inline'] .cm-cursor {
    border-left: var(--jp-code-cursor-width1) solid
      var(--jp-editor-cursor-color);
  }
}

/* When zoomed out less than 33% */
@media screen and (min-width: 4320px) {
  .jp-CodeMirrorEditor[data-type='inline'] .cm-cursor {
    border-left: var(--jp-code-cursor-width2) solid
      var(--jp-editor-cursor-color);
  }
}

.cm-editor.jp-mod-readOnly .cm-cursor {
  display: none;
}

.jp-CollaboratorCursor {
  border-left: 5px solid transparent;
  border-right: 5px solid transparent;
  border-top: none;
  border-bottom: 3px solid;
  background-clip: content-box;
  margin-left: -5px;
  margin-right: -5px;
}

.cm-searching,
.cm-searching span {
  /* `.cm-searching span`: we need to override syntax highlighting */
  background-color: var(--jp-search-unselected-match-background-color);
  color: var(--jp-search-unselected-match-color);
}

.cm-searching::selection,
.cm-searching span::selection {
  background-color: var(--jp-search-unselected-match-background-color);
  color: var(--jp-search-unselected-match-color);
}

.jp-current-match > .cm-searching,
.jp-current-match > .cm-searching span,
.cm-searching > .jp-current-match,
.cm-searching > .jp-current-match span {
  background-color: var(--jp-search-selected-match-background-color);
  color: var(--jp-search-selected-match-color);
}

.jp-current-match > .cm-searching::selection,
.cm-searching > .jp-current-match::selection,
.jp-current-match > .cm-searching span::selection {
  background-color: var(--jp-search-selected-match-background-color);
  color: var(--jp-search-selected-match-color);
}

.cm-trailingspace {
  background-image: url(data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAAgAAAAFCAYAAAB4ka1VAAAAsElEQVQIHQGlAFr/AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA7+r3zKmT0/+pk9P/7+r3zAAAAAAAAAAABAAAAAAAAAAA6OPzM+/q9wAAAAAA6OPzMwAAAAAAAAAAAgAAAAAAAAAAGR8NiRQaCgAZIA0AGR8NiQAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAQyoYJ/SY80UAAAAASUVORK5CYII=);
  background-position: center left;
  background-repeat: repeat-x;
}

.jp-CollaboratorCursor-hover {
  position: absolute;
  z-index: 1;
  transform: translateX(-50%);
  color: white;
  border-radius: 3px;
  padding-left: 4px;
  padding-right: 4px;
  padding-top: 1px;
  padding-bottom: 1px;
  text-align: center;
  font-size: var(--jp-ui-font-size1);
  white-space: nowrap;
}

.jp-CodeMirror-ruler {
  border-left: 1px dashed var(--jp-border-color2);
}

/* Styles for shared cursors (remote cursor locations and selected ranges) */
.jp-CodeMirrorEditor .cm-ySelectionCaret {
  position: relative;
  border-left: 1px solid black;
  margin-left: -1px;
  margin-right: -1px;
  box-sizing: border-box;
}

.jp-CodeMirrorEditor .cm-ySelectionCaret > .cm-ySelectionInfo {
  white-space: nowrap;
  position: absolute;
  top: -1.15em;
  padding-bottom: 0.05em;
  left: -1px;
  font-size: 0.95em;
  font-family: var(--jp-ui-font-family);
  font-weight: bold;
  line-height: normal;
  user-select: none;
  color: white;
  padding-left: 2px;
  padding-right: 2px;
  z-index: 101;
  transition: opacity 0.3s ease-in-out;
}

.jp-CodeMirrorEditor .cm-ySelectionInfo {
  transition-delay: 0.7s;
  opacity: 0;
}

.jp-CodeMirrorEditor .cm-ySelectionCaret:hover > .cm-ySelectionInfo {
  opacity: 1;
  transition-delay: 0s;
}

/*-----------------------------------------------------------------------------
| Copyright (c) Jupyter Development Team.
| Distributed under the terms of the Modified BSD License.
|----------------------------------------------------------------------------*/

.jp-MimeDocument {
  outline: none;
}

/*-----------------------------------------------------------------------------
| Copyright (c) Jupyter Development Team.
| Distributed under the terms of the Modified BSD License.
|----------------------------------------------------------------------------*/

/*-----------------------------------------------------------------------------
| Variables
|----------------------------------------------------------------------------*/

:root {
  --jp-private-filebrowser-button-height: 28px;
  --jp-private-filebrowser-button-width: 48px;
}

/*-----------------------------------------------------------------------------
| Copyright (c) Jupyter Development Team.
| Distributed under the terms of the Modified BSD License.
|----------------------------------------------------------------------------*/

.jp-FileBrowser .jp-SidePanel-content {
  display: flex;
  flex-direction: column;
}

.jp-FileBrowser-toolbar.jp-Toolbar {
  flex-wrap: wrap;
  row-gap: 12px;
  border-bottom: none;
  height: auto;
  margin: 8px 12px 0;
  box-shadow: none;
  padding: 0;
  justify-content: flex-start;
}

.jp-FileBrowser-Panel {
  flex: 1 1 auto;
  display: flex;
  flex-direction: column;
}

.jp-BreadCrumbs {
  flex: 0 0 auto;
  margin: 8px 12px;
}

.jp-BreadCrumbs-item {
  margin: 0 2px;
  padding: 0 2px;
  border-radius: var(--jp-border-radius);
  cursor: pointer;
}

.jp-BreadCrumbs-item:hover {
  background-color: var(--jp-layout-color2);
}

.jp-BreadCrumbs-item:first-child {
  margin-left: 0;
}

.jp-BreadCrumbs-item.jp-mod-dropTarget {
  background-color: var(--jp-brand-color2);
  opacity: 0.7;
}

/*-----------------------------------------------------------------------------
| Buttons
|----------------------------------------------------------------------------*/

.jp-FileBrowser-toolbar > .jp-Toolbar-item {
  flex: 0 0 auto;
  padding-left: 0;
  padding-right: 2px;
  align-items: center;
  height: unset;
}

.jp-FileBrowser-toolbar > .jp-Toolbar-item .jp-ToolbarButtonComponent {
  width: 40px;
}

/*-----------------------------------------------------------------------------
| Other styles
|----------------------------------------------------------------------------*/

.jp-FileDialog.jp-mod-conflict input {
  color: var(--jp-error-color1);
}

.jp-FileDialog .jp-new-name-title {
  margin-top: 12px;
}

.jp-LastModified-hidden {
  display: none;
}

.jp-FileSize-hidden {
  display: none;
}

.jp-FileBrowser .lm-AccordionPanel > h3:first-child {
  display: none;
}

/*-----------------------------------------------------------------------------
| DirListing
|----------------------------------------------------------------------------*/

.jp-DirListing {
  flex: 1 1 auto;
  display: flex;
  flex-direction: column;
  outline: 0;
}

.jp-DirListing-header {
  flex: 0 0 auto;
  display: flex;
  flex-direction: row;
  align-items: center;
  overflow: hidden;
  border-top: var(--jp-border-width) solid var(--jp-border-color2);
  border-bottom: var(--jp-border-width) solid var(--jp-border-color1);
  box-shadow: var(--jp-toolbar-box-shadow);
  z-index: 2;
}

.jp-DirListing-headerItem {
  padding: 4px 12px 2px;
  font-weight: 500;
}

.jp-DirListing-headerItem:hover {
  background: var(--jp-layout-color2);
}

.jp-DirListing-headerItem.jp-id-name {
  flex: 1 0 84px;
}

.jp-DirListing-headerItem.jp-id-modified {
  flex: 0 0 112px;
  border-left: var(--jp-border-width) solid var(--jp-border-color2);
  text-align: right;
}

.jp-DirListing-headerItem.jp-id-filesize {
  flex: 0 0 75px;
  border-left: var(--jp-border-width) solid var(--jp-border-color2);
  text-align: right;
}

.jp-id-narrow {
  display: none;
  flex: 0 0 5px;
  padding: 4px;
  border-left: var(--jp-border-width) solid var(--jp-border-color2);
  text-align: right;
  color: var(--jp-border-color2);
}

.jp-DirListing-narrow .jp-id-narrow {
  display: block;
}

.jp-DirListing-narrow .jp-id-modified,
.jp-DirListing-narrow .jp-DirListing-itemModified {
  display: none;
}

.jp-DirListing-headerItem.jp-mod-selected {
  font-weight: 600;
}

/* increase specificity to override bundled default */
.jp-DirListing-content {
  flex: 1 1 auto;
  margin: 0;
  padding: 0;
  list-style-type: none;
  overflow: auto;
  background-color: var(--jp-layout-color1);
}

.jp-DirListing-content mark {
  color: var(--jp-ui-font-color0);
  background-color: transparent;
  font-weight: bold;
}

.jp-DirListing-content .jp-DirListing-item.jp-mod-selected mark {
  color: var(--jp-ui-inverse-font-color0);
}

/* Style the directory listing content when a user drops a file to upload */
.jp-DirListing.jp-mod-native-drop .jp-DirListing-content {
  outline: 5px dashed rgba(128, 128, 128, 0.5);
  outline-offset: -10px;
  cursor: copy;
}

.jp-DirListing-item {
  display: flex;
  flex-direction: row;
  align-items: center;
  padding: 4px 12px;
  -webkit-user-select: none;
  -moz-user-select: none;
  -ms-user-select: none;
  user-select: none;
}

.jp-DirListing-checkboxWrapper {
  /* Increases hit area of checkbox. */
  padding: 4px;
}

.jp-DirListing-header
  .jp-DirListing-checkboxWrapper
  + .jp-DirListing-headerItem {
  padding-left: 4px;
}

.jp-DirListing-content .jp-DirListing-checkboxWrapper {
  position: relative;
  left: -4px;
  margin: -4px 0 -4px -8px;
}

.jp-DirListing-checkboxWrapper.jp-mod-visible {
  visibility: visible;
}

/* For devices that support hovering, hide checkboxes until hovered, selected...
*/
@media (hover: hover) {
  .jp-DirListing-checkboxWrapper {
    visibility: hidden;
  }

  .jp-DirListing-item:hover .jp-DirListing-checkboxWrapper,
  .jp-DirListing-item.jp-mod-selected .jp-DirListing-checkboxWrapper {
    visibility: visible;
  }
}

.jp-DirListing-item[data-is-dot] {
  opacity: 75%;
}

.jp-DirListing-item.jp-mod-selected {
  color: var(--jp-ui-inverse-font-color1);
  background: var(--jp-brand-color1);
}

.jp-DirListing-item.jp-mod-dropTarget {
  background: var(--jp-brand-color3);
}

.jp-DirListing-item:hover:not(.jp-mod-selected) {
  background: var(--jp-layout-color2);
}

.jp-DirListing-itemIcon {
  flex: 0 0 20px;
  margin-right: 4px;
}

.jp-DirListing-itemText {
  flex: 1 0 64px;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
  user-select: none;
}

.jp-DirListing-itemText:focus {
  outline-width: 2px;
  outline-color: var(--jp-inverse-layout-color1);
  outline-style: solid;
  outline-offset: 1px;
}

.jp-DirListing-item.jp-mod-selected .jp-DirListing-itemText:focus {
  outline-color: var(--jp-layout-color1);
}

.jp-DirListing-itemModified {
  flex: 0 0 125px;
  text-align: right;
}

.jp-DirListing-itemFileSize {
  flex: 0 0 90px;
  text-align: right;
}

.jp-DirListing-editor {
  flex: 1 0 64px;
  outline: none;
  border: none;
  color: var(--jp-ui-font-color1);
  background-color: var(--jp-layout-color1);
}

.jp-DirListing-item.jp-mod-running .jp-DirListing-itemIcon::before {
  color: var(--jp-success-color1);
  content: '\25CF';
  font-size: 8px;
  position: absolute;
  left: -8px;
}

.jp-DirListing-item.jp-mod-running.jp-mod-selected
  .jp-DirListing-itemIcon::before {
  color: var(--jp-ui-inverse-font-color1);
}

.jp-DirListing-item.lm-mod-drag-image,
.jp-DirListing-item.jp-mod-selected.lm-mod-drag-image {
  font-size: var(--jp-ui-font-size1);
  padding-left: 4px;
  margin-left: 4px;
  width: 160px;
  background-color: var(--jp-ui-inverse-font-color2);
  box-shadow: var(--jp-elevation-z2);
  border-radius: 0;
  color: var(--jp-ui-font-color1);
  transform: translateX(-40%) translateY(-58%);
}

.jp-Document {
  min-width: 120px;
  min-height: 120px;
  outline: none;
}

/*-----------------------------------------------------------------------------
| Copyright (c) Jupyter Development Team.
| Distributed under the terms of the Modified BSD License.
|----------------------------------------------------------------------------*/

/*-----------------------------------------------------------------------------
| Main OutputArea
| OutputArea has a list of Outputs
|----------------------------------------------------------------------------*/

.jp-OutputArea {
  overflow-y: auto;
}

.jp-OutputArea-child {
  display: table;
  table-layout: fixed;
  width: 100%;
  overflow: hidden;
}

.jp-OutputPrompt {
  width: var(--jp-cell-prompt-width);
  color: var(--jp-cell-outprompt-font-color);
  font-family: var(--jp-cell-prompt-font-family);
  padding: var(--jp-code-padding);
  letter-spacing: var(--jp-cell-prompt-letter-spacing);
  line-height: var(--jp-code-line-height);
  font-size: var(--jp-code-font-size);
  border: var(--jp-border-width) solid transparent;
  opacity: var(--jp-cell-prompt-opacity);

  /* Right align prompt text, don't wrap to handle large prompt numbers */
  text-align: right;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;

  /* Disable text selection */
  -webkit-user-select: none;
  -moz-user-select: none;
  -ms-user-select: none;
  user-select: none;
}

.jp-OutputArea-prompt {
  display: table-cell;
  vertical-align: top;
}

.jp-OutputArea-output {
  display: table-cell;
  width: 100%;
  height: auto;
  overflow: auto;
  user-select: text;
  -moz-user-select: text;
  -webkit-user-select: text;
  -ms-user-select: text;
}

.jp-OutputArea .jp-RenderedText {
  padding-left: 1ch;
}

/**
 * Prompt overlay.
 */

.jp-OutputArea-promptOverlay {
  position: absolute;
  top: 0;
  width: var(--jp-cell-prompt-width);
  height: 100%;
  opacity: 0.5;
}

.jp-OutputArea-promptOverlay:hover {
  background: var(--jp-layout-color2);
  box-shadow: inset 0 0 1px var(--jp-inverse-layout-color0);
  cursor: zoom-out;
}

.jp-mod-outputsScrolled .jp-OutputArea-promptOverlay:hover {
  cursor: zoom-in;
}

/**
 * Isolated output.
 */
.jp-OutputArea-output.jp-mod-isolated {
  width: 100%;
  display: block;
}

/*
When drag events occur, `lm-mod-override-cursor` is added to the body.
Because iframes steal all cursor events, the following two rules are necessary
to suppress pointer events while resize drags are occurring. There may be a
better solution to this problem.
*/
body.lm-mod-override-cursor .jp-OutputArea-output.jp-mod-isolated {
  position: relative;
}

body.lm-mod-override-cursor .jp-OutputArea-output.jp-mod-isolated::before {
  content: '';
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: transparent;
}

/* pre */

.jp-OutputArea-output pre {
  border: none;
  margin: 0;
  padding: 0;
  overflow-x: auto;
  overflow-y: auto;
  word-break: break-all;
  word-wrap: break-word;
  white-space: pre-wrap;
}

/* tables */

.jp-OutputArea-output.jp-RenderedHTMLCommon table {
  margin-left: 0;
  margin-right: 0;
}

/* description lists */

.jp-OutputArea-output dl,
.jp-OutputArea-output dt,
.jp-OutputArea-output dd {
  display: block;
}

.jp-OutputArea-output dl {
  width: 100%;
  overflow: hidden;
  padding: 0;
  margin: 0;
}

.jp-OutputArea-output dt {
  font-weight: bold;
  float: left;
  width: 20%;
  padding: 0;
  margin: 0;
}

.jp-OutputArea-output dd {
  float: left;
  width: 80%;
  padding: 0;
  margin: 0;
}

.jp-TrimmedOutputs pre {
  background: var(--jp-layout-color3);
  font-size: calc(var(--jp-code-font-size) * 1.4);
  text-align: center;
  text-transform: uppercase;
}

/* Hide the gutter in case of
 *  - nested output areas (e.g. in the case of output widgets)
 *  - mirrored output areas
 */
.jp-OutputArea .jp-OutputArea .jp-OutputArea-prompt {
  display: none;
}

/* Hide empty lines in the output area, for instance due to cleared widgets */
.jp-OutputArea-prompt:empty {
  padding: 0;
  border: 0;
}

/*-----------------------------------------------------------------------------
| executeResult is added to any Output-result for the display of the object
| returned by a cell
|----------------------------------------------------------------------------*/

.jp-OutputArea-output.jp-OutputArea-executeResult {
  margin-left: 0;
  width: 100%;
}

/* Text output with the Out[] prompt needs a top padding to match the
 * alignment of the Out[] prompt itself.
 */
.jp-OutputArea-executeResult .jp-RenderedText.jp-OutputArea-output {
  padding-top: var(--jp-code-padding);
  border-top: var(--jp-border-width) solid transparent;
}

/*-----------------------------------------------------------------------------
| The Stdin output
|----------------------------------------------------------------------------*/

.jp-Stdin-prompt {
  color: var(--jp-content-font-color0);
  padding-right: var(--jp-code-padding);
  vertical-align: baseline;
  flex: 0 0 auto;
}

.jp-Stdin-input {
  font-family: var(--jp-code-font-family);
  font-size: inherit;
  color: inherit;
  background-color: inherit;
  width: 42%;
  min-width: 200px;

  /* make sure input baseline aligns with prompt */
  vertical-align: baseline;

  /* padding + margin = 0.5em between prompt and cursor */
  padding: 0 0.25em;
  margin: 0 0.25em;
  flex: 0 0 70%;
}

.jp-Stdin-input::placeholder {
  opacity: 0;
}

.jp-Stdin-input:focus {
  box-shadow: none;
}

.jp-Stdin-input:focus::placeholder {
  opacity: 1;
}

/*-----------------------------------------------------------------------------
| Output Area View
|----------------------------------------------------------------------------*/

.jp-LinkedOutputView .jp-OutputArea {
  height: 100%;
  display: block;
}

.jp-LinkedOutputView .jp-OutputArea-output:only-child {
  height: 100%;
}

/*-----------------------------------------------------------------------------
| Printing
|----------------------------------------------------------------------------*/

@media print {
  .jp-OutputArea-child {
    break-inside: avoid-page;
  }
}

/*-----------------------------------------------------------------------------
| Mobile
|----------------------------------------------------------------------------*/
@media only screen and (max-width: 760px) {
  .jp-OutputPrompt {
    display: table-row;
    text-align: left;
  }

  .jp-OutputArea-child .jp-OutputArea-output {
    display: table-row;
    margin-left: var(--jp-notebook-padding);
  }
}

/* Trimmed outputs warning */
.jp-TrimmedOutputs > a {
  margin: 10px;
  text-decoration: none;
  cursor: pointer;
}

.jp-TrimmedOutputs > a:hover {
  text-decoration: none;
}

/*-----------------------------------------------------------------------------
| Copyright (c) Jupyter Development Team.
| Distributed under the terms of the Modified BSD License.
|----------------------------------------------------------------------------*/

/*-----------------------------------------------------------------------------
| Table of Contents
|----------------------------------------------------------------------------*/

:root {
  --jp-private-toc-active-width: 4px;
}

.jp-TableOfContents {
  display: flex;
  flex-direction: column;
  background: var(--jp-layout-color1);
  color: var(--jp-ui-font-color1);
  font-size: var(--jp-ui-font-size1);
  height: 100%;
}

.jp-TableOfContents-placeholder {
  text-align: center;
}

.jp-TableOfContents-placeholderContent {
  color: var(--jp-content-font-color2);
  padding: 8px;
}

.jp-TableOfContents-placeholderContent > h3 {
  margin-bottom: var(--jp-content-heading-margin-bottom);
}

.jp-TableOfContents .jp-SidePanel-content {
  overflow-y: auto;
}

.jp-TableOfContents-tree {
  margin: 4px;
}

.jp-TableOfContents ol {
  list-style-type: none;
}

/* stylelint-disable-next-line selector-max-type */
.jp-TableOfContents li > ol {
  /* Align left border with triangle icon center */
  padding-left: 11px;
}

.jp-TableOfContents-content {
  /* left margin for the active heading indicator */
  margin: 0 0 0 var(--jp-private-toc-active-width);
  padding: 0;
  background-color: var(--jp-layout-color1);
}

.jp-tocItem {
  -webkit-user-select: none;
  -moz-user-select: none;
  -ms-user-select: none;
  user-select: none;
}

.jp-tocItem-heading {
  display: flex;
  cursor: pointer;
}

.jp-tocItem-heading:hover {
  background-color: var(--jp-layout-color2);
}

.jp-tocItem-content {
  display: block;
  padding: 4px 0;
  white-space: nowrap;
  text-overflow: ellipsis;
  overflow-x: hidden;
}

.jp-tocItem-collapser {
  height: 20px;
  margin: 2px 2px 0;
  padding: 0;
  background: none;
  border: none;
  cursor: pointer;
}

.jp-tocItem-collapser:hover {
  background-color: var(--jp-layout-color3);
}

/* Active heading indicator */

.jp-tocItem-heading::before {
  content: ' ';
  background: transparent;
  width: var(--jp-private-toc-active-width);
  height: 24px;
  position: absolute;
  left: 0;
  border-radius: var(--jp-border-radius);
}

.jp-tocItem-heading.jp-tocItem-active::before {
  background-color: var(--jp-brand-color1);
}

.jp-tocItem-heading:hover.jp-tocItem-active::before {
  background: var(--jp-brand-color0);
  opacity: 1;
}

/*-----------------------------------------------------------------------------
| Copyright (c) Jupyter Development Team.
| Distributed under the terms of the Modified BSD License.
|----------------------------------------------------------------------------*/

.jp-Collapser {
  flex: 0 0 var(--jp-cell-collapser-width);
  padding: 0;
  margin: 0;
  border: none;
  outline: none;
  background: transparent;
  border-radius: var(--jp-border-radius);
  opacity: 1;
}

.jp-Collapser-child {
  display: block;
  width: 100%;
  box-sizing: border-box;

  /* height: 100% doesn't work because the height of its parent is computed from content */
  position: absolute;
  top: 0;
  bottom: 0;
}

/*-----------------------------------------------------------------------------
| Printing
|----------------------------------------------------------------------------*/

/*
Hiding collapsers in print mode.

Note: input and output wrappers have "display: block" propery in print mode.
*/

@media print {
  .jp-Collapser {
    display: none;
  }
}

/*-----------------------------------------------------------------------------
| Copyright (c) Jupyter Development Team.
| Distributed under the terms of the Modified BSD License.
|----------------------------------------------------------------------------*/

/*-----------------------------------------------------------------------------
| Header/Footer
|----------------------------------------------------------------------------*/

/* Hidden by zero height by default */
.jp-CellHeader,
.jp-CellFooter {
  height: 0;
  width: 100%;
  padding: 0;
  margin: 0;
  border: none;
  outline: none;
  background: transparent;
}

/*-----------------------------------------------------------------------------
| Copyright (c) Jupyter Development Team.
| Distributed under the terms of the Modified BSD License.
|----------------------------------------------------------------------------*/

/*-----------------------------------------------------------------------------
| Input
|----------------------------------------------------------------------------*/

/* All input areas */
.jp-InputArea {
  display: table;
  table-layout: fixed;
  width: 100%;
  overflow: hidden;
}

.jp-InputArea-editor {
  display: table-cell;
  overflow: hidden;
  vertical-align: top;

  /* This is the non-active, default styling */
  border: var(--jp-border-width) solid var(--jp-cell-editor-border-color);
  border-radius: 0;
  background: var(--jp-cell-editor-background);
}

.jp-InputPrompt {
  display: table-cell;
  vertical-align: top;
  width: var(--jp-cell-prompt-width);
  color: var(--jp-cell-inprompt-font-color);
  font-family: var(--jp-cell-prompt-font-family);
  padding: var(--jp-code-padding);
  letter-spacing: var(--jp-cell-prompt-letter-spacing);
  opacity: var(--jp-cell-prompt-opacity);
  line-height: var(--jp-code-line-height);
  font-size: var(--jp-code-font-size);
  border: var(--jp-border-width) solid transparent;

  /* Right align prompt text, don't wrap to handle large prompt numbers */
  text-align: right;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;

  /* Disable text selection */
  -webkit-user-select: none;
  -moz-user-select: none;
  -ms-user-select: none;
  user-select: none;
}

/*-----------------------------------------------------------------------------
| Mobile
|----------------------------------------------------------------------------*/
@media only screen and (max-width: 760px) {
  .jp-InputArea-editor {
    display: table-row;
    margin-left: var(--jp-notebook-padding);
  }

  .jp-InputPrompt {
    display: table-row;
    text-align: left;
  }
}

/*-----------------------------------------------------------------------------
| Copyright (c) Jupyter Development Team.
| Distributed under the terms of the Modified BSD License.
|----------------------------------------------------------------------------*/

/*-----------------------------------------------------------------------------
| Placeholder
|----------------------------------------------------------------------------*/

.jp-Placeholder {
  display: table;
  table-layout: fixed;
  width: 100%;
}

.jp-Placeholder-prompt {
  display: table-cell;
  box-sizing: border-box;
}

.jp-Placeholder-content {
  display: table-cell;
  padding: 4px 6px;
  border: 1px solid transparent;
  border-radius: 0;
  background: none;
  box-sizing: border-box;
  cursor: pointer;
}

.jp-Placeholder-contentContainer {
  display: flex;
}

.jp-Placeholder-content:hover,
.jp-InputPlaceholder > .jp-Placeholder-content:hover {
  border-color: var(--jp-layout-color3);
}

.jp-Placeholder-content .jp-MoreHorizIcon {
  width: 32px;
  height: 16px;
  border: 1px solid transparent;
  border-radius: var(--jp-border-radius);
}

.jp-Placeholder-content .jp-MoreHorizIcon:hover {
  border: 1px solid var(--jp-border-color1);
  box-shadow: 0 0 2px 0 rgba(0, 0, 0, 0.25);
  background-color: var(--jp-layout-color0);
}

.jp-PlaceholderText {
  white-space: nowrap;
  overflow-x: hidden;
  color: var(--jp-inverse-layout-color3);
  font-family: var(--jp-code-font-family);
}

.jp-InputPlaceholder > .jp-Placeholder-content {
  border-color: var(--jp-cell-editor-border-color);
  background: var(--jp-cell-editor-background);
}

/*-----------------------------------------------------------------------------
| Copyright (c) Jupyter Development Team.
| Distributed under the terms of the Modified BSD License.
|----------------------------------------------------------------------------*/

/*-----------------------------------------------------------------------------
| Private CSS variables
|----------------------------------------------------------------------------*/

:root {
  --jp-private-cell-scrolling-output-offset: 5px;
}

/*-----------------------------------------------------------------------------
| Cell
|----------------------------------------------------------------------------*/

.jp-Cell {
  padding: var(--jp-cell-padding);
  margin: 0;
  border: none;
  outline: none;
  background: transparent;
}

/*-----------------------------------------------------------------------------
| Common input/output
|----------------------------------------------------------------------------*/

.jp-Cell-inputWrapper,
.jp-Cell-outputWrapper {
  display: flex;
  flex-direction: row;
  padding: 0;
  margin: 0;

  /* Added to reveal the box-shadow on the input and output collapsers. */
  overflow: visible;
}

/* Only input/output areas inside cells */
.jp-Cell-inputArea,
.jp-Cell-outputArea {
  flex: 1 1 auto;
}

/*-----------------------------------------------------------------------------
| Collapser
|----------------------------------------------------------------------------*/

/* Make the output collapser disappear when there is not output, but do so
 * in a manner that leaves it in the layout and preserves its width.
 */
.jp-Cell.jp-mod-noOutputs .jp-Cell-outputCollapser {
  border: none !important;
  background: transparent !important;
}

.jp-Cell:not(.jp-mod-noOutputs) .jp-Cell-outputCollapser {
  min-height: var(--jp-cell-collapser-min-height);
}

/*-----------------------------------------------------------------------------
| Output
|----------------------------------------------------------------------------*/

/* Put a space between input and output when there IS output */
.jp-Cell:not(.jp-mod-noOutputs) .jp-Cell-outputWrapper {
  margin-top: 5px;
}

.jp-CodeCell.jp-mod-outputsScrolled .jp-Cell-outputArea {
  overflow-y: auto;
  max-height: 24em;
  margin-left: var(--jp-private-cell-scrolling-output-offset);
  resize: vertical;
}

.jp-CodeCell.jp-mod-outputsScrolled .jp-Cell-outputArea[style*='height'] {
  max-height: unset;
}

.jp-CodeCell.jp-mod-outputsScrolled .jp-Cell-outputArea::after {
  content: ' ';
  box-shadow: inset 0 0 6px 2px rgb(0 0 0 / 30%);
  width: 100%;
  height: 100%;
  position: sticky;
  bottom: 0;
  top: 0;
  margin-top: -50%;
  float: left;
  display: block;
  pointer-events: none;
}

.jp-CodeCell.jp-mod-outputsScrolled .jp-OutputArea-child {
  padding-top: 6px;
}

.jp-CodeCell.jp-mod-outputsScrolled .jp-OutputArea-prompt {
  width: calc(
    var(--jp-cell-prompt-width) - var(--jp-private-cell-scrolling-output-offset)
  );
}

.jp-CodeCell.jp-mod-outputsScrolled .jp-OutputArea-promptOverlay {
  left: calc(-1 * var(--jp-private-cell-scrolling-output-offset));
}

/*-----------------------------------------------------------------------------
| CodeCell
|----------------------------------------------------------------------------*/

/*-----------------------------------------------------------------------------
| MarkdownCell
|----------------------------------------------------------------------------*/

.jp-MarkdownOutput {
  display: table-cell;
  width: 100%;
  margin-top: 0;
  margin-bottom: 0;
  padding-left: var(--jp-code-padding);
}

.jp-MarkdownOutput.jp-RenderedHTMLCommon {
  overflow: auto;
}

/* collapseHeadingButton (show always if hiddenCellsButton is _not_ shown) */
.jp-collapseHeadingButton {
  display: flex;
  min-height: var(--jp-cell-collapser-min-height);
  font-size: var(--jp-code-font-size);
  position: absolute;
  background-color: transparent;
  background-size: 25px;
  background-repeat: no-repeat;
  background-position-x: center;
  background-position-y: top;
  background-image: var(--jp-icon-caret-down);
  right: 0;
  top: 0;
  bottom: 0;
}

.jp-collapseHeadingButton.jp-mod-collapsed {
  background-image: var(--jp-icon-caret-right);
}

/*
 set the container font size to match that of content
 so that the nested collapse buttons have the right size
*/
.jp-MarkdownCell .jp-InputPrompt {
  font-size: var(--jp-content-font-size1);
}

/*
  Align collapseHeadingButton with cell top header
  The font sizes are identical to the ones in packages/rendermime/style/base.css
*/
.jp-mod-rendered .jp-collapseHeadingButton[data-heading-level='1'] {
  font-size: var(--jp-content-font-size5);
  background-position-y: calc(0.3 * var(--jp-content-font-size5));
}

.jp-mod-rendered .jp-collapseHeadingButton[data-heading-level='2'] {
  font-size: var(--jp-content-font-size4);
  background-position-y: calc(0.3 * var(--jp-content-font-size4));
}

.jp-mod-rendered .jp-collapseHeadingButton[data-heading-level='3'] {
  font-size: var(--jp-content-font-size3);
  background-position-y: calc(0.3 * var(--jp-content-font-size3));
}

.jp-mod-rendered .jp-collapseHeadingButton[data-heading-level='4'] {
  font-size: var(--jp-content-font-size2);
  background-position-y: calc(0.3 * var(--jp-content-font-size2));
}

.jp-mod-rendered .jp-collapseHeadingButton[data-heading-level='5'] {
  font-size: var(--jp-content-font-size1);
  background-position-y: top;
}

.jp-mod-rendered .jp-collapseHeadingButton[data-heading-level='6'] {
  font-size: var(--jp-content-font-size0);
  background-position-y: top;
}

/* collapseHeadingButton (show only on (hover,active) if hiddenCellsButton is shown) */
.jp-Notebook.jp-mod-showHiddenCellsButton .jp-collapseHeadingButton {
  display: none;
}

.jp-Notebook.jp-mod-showHiddenCellsButton
  :is(.jp-MarkdownCell:hover, .jp-mod-active)
  .jp-collapseHeadingButton {
  display: flex;
}

/* showHiddenCellsButton (only show if jp-mod-showHiddenCellsButton is set, which
is a consequence of the showHiddenCellsButton option in Notebook Settings)*/
.jp-Notebook.jp-mod-showHiddenCellsButton .jp-showHiddenCellsButton {
  margin-left: calc(var(--jp-cell-prompt-width) + 2 * var(--jp-code-padding));
  margin-top: var(--jp-code-padding);
  border: 1px solid var(--jp-border-color2);
  background-color: var(--jp-border-color3) !important;
  color: var(--jp-content-font-color0) !important;
  display: flex;
}

.jp-Notebook.jp-mod-showHiddenCellsButton .jp-showHiddenCellsButton:hover {
  background-color: var(--jp-border-color2) !important;
}

.jp-showHiddenCellsButton {
  display: none;
}

/*-----------------------------------------------------------------------------
| Printing
|----------------------------------------------------------------------------*/

/*
Using block instead of flex to allow the use of the break-inside CSS property for
cell outputs.
*/

@media print {
  .jp-Cell-inputWrapper,
  .jp-Cell-outputWrapper {
    display: block;
  }
}

/*-----------------------------------------------------------------------------
| Copyright (c) Jupyter Development Team.
| Distributed under the terms of the Modified BSD License.
|----------------------------------------------------------------------------*/

/*-----------------------------------------------------------------------------
| Copyright (c) Jupyter Development Team.
| Distributed under the terms of the Modified BSD License.
|----------------------------------------------------------------------------*/

/*-----------------------------------------------------------------------------
| Variables
|----------------------------------------------------------------------------*/

:root {
  --jp-notebook-toolbar-padding: 2px 5px 2px 2px;
}

/*-----------------------------------------------------------------------------

/*-----------------------------------------------------------------------------
| Styles
|----------------------------------------------------------------------------*/

.jp-NotebookPanel-toolbar {
  padding: var(--jp-notebook-toolbar-padding);

  /* disable paint containment from lumino 2.0 default strict CSS containment */
  contain: style size !important;
}

.jp-Toolbar-item.jp-Notebook-toolbarCellType .jp-select-wrapper.jp-mod-focused {
  border: none;
  box-shadow: none;
}

.jp-Notebook-toolbarCellTypeDropdown select {
  height: 24px;
  font-size: var(--jp-ui-font-size1);
  line-height: 14px;
  border-radius: 0;
  display: block;
}

.jp-Notebook-toolbarCellTypeDropdown span {
  top: 5px !important;
}

.jp-Toolbar-responsive-popup {
  position: absolute;
  height: fit-content;
  display: flex;
  flex-direction: row;
  flex-wrap: wrap;
  justify-content: flex-end;
  border-bottom: var(--jp-border-width) solid var(--jp-toolbar-border-color);
  box-shadow: var(--jp-toolbar-box-shadow);
  background: var(--jp-toolbar-background);
  min-height: var(--jp-toolbar-micro-height);
  padding: var(--jp-notebook-toolbar-padding);
  z-index: 1;
  right: 0;
  top: 0;
}

.jp-Toolbar > .jp-Toolbar-responsive-opener {
  margin-left: auto;
}

/*-----------------------------------------------------------------------------
| Copyright (c) Jupyter Development Team.
| Distributed under the terms of the Modified BSD License.
|----------------------------------------------------------------------------*/

/*-----------------------------------------------------------------------------
| Variables
|----------------------------------------------------------------------------*/

/*-----------------------------------------------------------------------------

/*-----------------------------------------------------------------------------
| Styles
|----------------------------------------------------------------------------*/

.jp-Notebook-ExecutionIndicator {
  position: relative;
  display: inline-block;
  height: 100%;
  z-index: 9997;
}

.jp-Notebook-ExecutionIndicator-tooltip {
  visibility: hidden;
  height: auto;
  width: max-content;
  width: -moz-max-content;
  background-color: var(--jp-layout-color2);
  color: var(--jp-ui-font-color1);
  text-align: justify;
  border-radius: 6px;
  padding: 0 5px;
  position: fixed;
  display: table;
}

.jp-Notebook-ExecutionIndicator-tooltip.up {
  transform: translateX(-50%) translateY(-100%) translateY(-32px);
}

.jp-Notebook-ExecutionIndicator-tooltip.down {
  transform: translateX(calc(-100% + 16px)) translateY(5px);
}

.jp-Notebook-ExecutionIndicator-tooltip.hidden {
  display: none;
}

.jp-Notebook-ExecutionIndicator:hover .jp-Notebook-ExecutionIndicator-tooltip {
  visibility: visible;
}

.jp-Notebook-ExecutionIndicator span {
  font-size: var(--jp-ui-font-size1);
  font-family: var(--jp-ui-font-family);
  color: var(--jp-ui-font-color1);
  line-height: 24px;
  display: block;
}

.jp-Notebook-ExecutionIndicator-progress-bar {
  display: flex;
  justify-content: center;
  height: 100%;
}

/*
 * Copyright (c) Jupyter Development Team.
 * Distributed under the terms of the Modified BSD License.
 */

/*
 * Execution indicator
 */
.jp-tocItem-content::after {
  content: '';

  /* Must be identical to form a circle */
  width: 12px;
  height: 12px;
  background: none;
  border: none;
  position: absolute;
  right: 0;
}

.jp-tocItem-content[data-running='0']::after {
  border-radius: 50%;
  border: var(--jp-border-width) solid var(--jp-inverse-layout-color3);
  background: none;
}

.jp-tocItem-content[data-running='1']::after {
  border-radius: 50%;
  border: var(--jp-border-width) solid var(--jp-inverse-layout-color3);
  background-color: var(--jp-inverse-layout-color3);
}

.jp-tocItem-content[data-running='0'],
.jp-tocItem-content[data-running='1'] {
  margin-right: 12px;
}

/*
 * Copyright (c) Jupyter Development Team.
 * Distributed under the terms of the Modified BSD License.
 */

.jp-Notebook-footer {
  height: 27px;
  margin-left: calc(
    var(--jp-cell-prompt-width) + var(--jp-cell-collapser-width) +
      var(--jp-cell-padding)
  );
  width: calc(
    100% -
      (
        var(--jp-cell-prompt-width) + var(--jp-cell-collapser-width) +
          var(--jp-cell-padding) + var(--jp-cell-padding)
      )
  );
  border: var(--jp-border-width) solid var(--jp-cell-editor-border-color);
  color: var(--jp-ui-font-color3);
  margin-top: 6px;
  background: none;
  cursor: pointer;
}

.jp-Notebook-footer:focus {
  border-color: var(--jp-cell-editor-active-border-color);
}

/* For devices that support hovering, hide footer until hover */
@media (hover: hover) {
  .jp-Notebook-footer {
    opacity: 0;
  }

  .jp-Notebook-footer:focus,
  .jp-Notebook-footer:hover {
    opacity: 1;
  }
}

/*-----------------------------------------------------------------------------
| Copyright (c) Jupyter Development Team.
| Distributed under the terms of the Modified BSD License.
|----------------------------------------------------------------------------*/

/*-----------------------------------------------------------------------------
| Imports
|----------------------------------------------------------------------------*/

/*-----------------------------------------------------------------------------
| CSS variables
|----------------------------------------------------------------------------*/

:root {
  --jp-side-by-side-output-size: 1fr;
  --jp-side-by-side-resized-cell: var(--jp-side-by-side-output-size);
  --jp-private-notebook-dragImage-width: 304px;
  --jp-private-notebook-dragImage-height: 36px;
  --jp-private-notebook-selected-color: var(--md-blue-400);
  --jp-private-notebook-active-color: var(--md-green-400);
}

/*-----------------------------------------------------------------------------
| Notebook
|----------------------------------------------------------------------------*/

/* stylelint-disable selector-max-class */

.jp-NotebookPanel {
  display: block;
  height: 100%;
}

.jp-NotebookPanel.jp-Document {
  min-width: 240px;
  min-height: 120px;
}

.jp-Notebook {
  padding: var(--jp-notebook-padding);
  outline: none;
  overflow: auto;
  background: var(--jp-layout-color0);
}

.jp-Notebook.jp-mod-scrollPastEnd::after {
  display: block;
  content: '';
  min-height: var(--jp-notebook-scroll-padding);
}

.jp-MainAreaWidget-ContainStrict .jp-Notebook * {
  contain: strict;
}

.jp-Notebook .jp-Cell {
  overflow: visible;
}

.jp-Notebook .jp-Cell .jp-InputPrompt {
  cursor: move;
}

/*-----------------------------------------------------------------------------
| Notebook state related styling
|
| The notebook and cells each have states, here are the possibilities:
|
| - Notebook
|   - Command
|   - Edit
| - Cell
|   - None
|   - Active (only one can be active)
|   - Selected (the cells actions are applied to)
|   - Multiselected (when multiple selected, the cursor)
|   - No outputs
|----------------------------------------------------------------------------*/

/* Command or edit modes */

.jp-Notebook .jp-Cell:not(.jp-mod-active) .jp-InputPrompt {
  opacity: var(--jp-cell-prompt-not-active-opacity);
  color: var(--jp-cell-prompt-not-active-font-color);
}

.jp-Notebook .jp-Cell:not(.jp-mod-active) .jp-OutputPrompt {
  opacity: var(--jp-cell-prompt-not-active-opacity);
  color: var(--jp-cell-prompt-not-active-font-color);
}

/* cell is active */
.jp-Notebook .jp-Cell.jp-mod-active .jp-Collapser {
  background: var(--jp-brand-color1);
}

/* cell is dirty */
.jp-Notebook .jp-Cell.jp-mod-dirty .jp-InputPrompt {
  color: var(--jp-warn-color1);
}

.jp-Notebook .jp-Cell.jp-mod-dirty .jp-InputPrompt::before {
  color: var(--jp-warn-color1);
  content: '•';
}

.jp-Notebook .jp-Cell.jp-mod-active.jp-mod-dirty .jp-Collapser {
  background: var(--jp-warn-color1);
}

/* collapser is hovered */
.jp-Notebook .jp-Cell .jp-Collapser:hover {
  box-shadow: var(--jp-elevation-z2);
  background: var(--jp-brand-color1);
  opacity: var(--jp-cell-collapser-not-active-hover-opacity);
}

/* cell is active and collapser is hovered */
.jp-Notebook .jp-Cell.jp-mod-active .jp-Collapser:hover {
  background: var(--jp-brand-color0);
  opacity: 1;
}

/* Command mode */

.jp-Notebook.jp-mod-commandMode .jp-Cell.jp-mod-selected {
  background: var(--jp-notebook-multiselected-color);
}

.jp-Notebook.jp-mod-commandMode
  .jp-Cell.jp-mod-active.jp-mod-selected:not(.jp-mod-multiSelected) {
  background: transparent;
}

/* Edit mode */

.jp-Notebook.jp-mod-editMode .jp-Cell.jp-mod-active .jp-InputArea-editor {
  border: var(--jp-border-width) solid var(--jp-cell-editor-active-border-color);
  box-shadow: var(--jp-input-box-shadow);
  background-color: var(--jp-cell-editor-active-background);
}

/*-----------------------------------------------------------------------------
| Notebook drag and drop
|----------------------------------------------------------------------------*/

.jp-Notebook-cell.jp-mod-dropSource {
  opacity: 0.5;
}

.jp-Notebook-cell.jp-mod-dropTarget,
.jp-Notebook.jp-mod-commandMode
  .jp-Notebook-cell.jp-mod-active.jp-mod-selected.jp-mod-dropTarget {
  border-top-color: var(--jp-private-notebook-selected-color);
  border-top-style: solid;
  border-top-width: 2px;
}

.jp-dragImage {
  display: block;
  flex-direction: row;
  width: var(--jp-private-notebook-dragImage-width);
  height: var(--jp-private-notebook-dragImage-height);
  border: var(--jp-border-width) solid var(--jp-cell-editor-border-color);
  background: var(--jp-cell-editor-background);
  overflow: visible;
}

.jp-dragImage-singlePrompt {
  box-shadow: 2px 2px 4px 0 rgba(0, 0, 0, 0.12);
}

.jp-dragImage .jp-dragImage-content {
  flex: 1 1 auto;
  z-index: 2;
  font-size: var(--jp-code-font-size);
  font-family: var(--jp-code-font-family);
  line-height: var(--jp-code-line-height);
  padding: var(--jp-code-padding);
  border: var(--jp-border-width) solid var(--jp-cell-editor-border-color);
  background: var(--jp-cell-editor-background-color);
  color: var(--jp-content-font-color3);
  text-align: left;
  margin: 4px 4px 4px 0;
}

.jp-dragImage .jp-dragImage-prompt {
  flex: 0 0 auto;
  min-width: 36px;
  color: var(--jp-cell-inprompt-font-color);
  padding: var(--jp-code-padding);
  padding-left: 12px;
  font-family: var(--jp-cell-prompt-font-family);
  letter-spacing: var(--jp-cell-prompt-letter-spacing);
  line-height: 1.9;
  font-size: var(--jp-code-font-size);
  border: var(--jp-border-width) solid transparent;
}

.jp-dragImage-multipleBack {
  z-index: -1;
  position: absolute;
  height: 32px;
  width: 300px;
  top: 8px;
  left: 8px;
  background: var(--jp-layout-color2);
  border: var(--jp-border-width) solid var(--jp-input-border-color);
  box-shadow: 2px 2px 4px 0 rgba(0, 0, 0, 0.12);
}

/*-----------------------------------------------------------------------------
| Cell toolbar
|----------------------------------------------------------------------------*/

.jp-NotebookTools {
  display: block;
  min-width: var(--jp-sidebar-min-width);
  color: var(--jp-ui-font-color1);
  background: var(--jp-layout-color1);

  /* This is needed so that all font sizing of children done in ems is
    * relative to this base size */
  font-size: var(--jp-ui-font-size1);
  overflow: auto;
}

.jp-ActiveCellTool {
  padding: 12px 0;
  display: flex;
}

.jp-ActiveCellTool-Content {
  flex: 1 1 auto;
}

.jp-ActiveCellTool .jp-ActiveCellTool-CellContent {
  background: var(--jp-cell-editor-background);
  border: var(--jp-border-width) solid var(--jp-cell-editor-border-color);
  border-radius: 0;
  min-height: 29px;
}

.jp-ActiveCellTool .jp-InputPrompt {
  min-width: calc(var(--jp-cell-prompt-width) * 0.75);
}

.jp-ActiveCellTool-CellContent > pre {
  padding: 5px 4px;
  margin: 0;
  white-space: normal;
}

.jp-MetadataEditorTool {
  flex-direction: column;
  padding: 12px 0;
}

.jp-RankedPanel > :not(:first-child) {
  margin-top: 12px;
}

.jp-KeySelector select.jp-mod-styled {
  font-size: var(--jp-ui-font-size1);
  color: var(--jp-ui-font-color0);
  border: var(--jp-border-width) solid var(--jp-border-color1);
}

.jp-KeySelector label,
.jp-MetadataEditorTool label,
.jp-NumberSetter label {
  line-height: 1.4;
}

.jp-NotebookTools .jp-select-wrapper {
  margin-top: 4px;
  margin-bottom: 0;
}

.jp-NumberSetter input {
  width: 100%;
  margin-top: 4px;
}

.jp-NotebookTools .jp-Collapse {
  margin-top: 16px;
}

/*-----------------------------------------------------------------------------
| Presentation Mode (.jp-mod-presentationMode)
|----------------------------------------------------------------------------*/

.jp-mod-presentationMode .jp-Notebook {
  --jp-content-font-size1: var(--jp-content-presentation-font-size1);
  --jp-code-font-size: var(--jp-code-presentation-font-size);
}

.jp-mod-presentationMode .jp-Notebook .jp-Cell .jp-InputPrompt,
.jp-mod-presentationMode .jp-Notebook .jp-Cell .jp-OutputPrompt {
  flex: 0 0 110px;
}

/*-----------------------------------------------------------------------------
| Side-by-side Mode (.jp-mod-sideBySide)
|----------------------------------------------------------------------------*/
.jp-mod-sideBySide.jp-Notebook .jp-Notebook-cell {
  margin-top: 3em;
  margin-bottom: 3em;
  margin-left: 5%;
  margin-right: 5%;
}

.jp-mod-sideBySide.jp-Notebook .jp-CodeCell {
  display: grid;
  grid-template-columns: minmax(0, 1fr) min-content minmax(
      0,
      var(--jp-side-by-side-output-size)
    );
  grid-template-rows: auto minmax(0, 1fr) auto;
  grid-template-areas:
    'header header header'
    'input handle output'
    'footer footer footer';
}

.jp-mod-sideBySide.jp-Notebook .jp-CodeCell.jp-mod-resizedCell {
  grid-template-columns: minmax(0, 1fr) min-content minmax(
      0,
      var(--jp-side-by-side-resized-cell)
    );
}

.jp-mod-sideBySide.jp-Notebook .jp-CodeCell .jp-CellHeader {
  grid-area: header;
}

.jp-mod-sideBySide.jp-Notebook .jp-CodeCell .jp-Cell-inputWrapper {
  grid-area: input;
}

.jp-mod-sideBySide.jp-Notebook .jp-CodeCell .jp-Cell-outputWrapper {
  /* overwrite the default margin (no vertical separation needed in side by side move */
  margin-top: 0;
  grid-area: output;
}

.jp-mod-sideBySide.jp-Notebook .jp-CodeCell .jp-CellFooter {
  grid-area: footer;
}

.jp-mod-sideBySide.jp-Notebook .jp-CodeCell .jp-CellResizeHandle {
  grid-area: handle;
  user-select: none;
  display: block;
  height: 100%;
  cursor: ew-resize;
  padding: 0 var(--jp-cell-padding);
}

.jp-mod-sideBySide.jp-Notebook .jp-CodeCell .jp-CellResizeHandle::after {
  content: '';
  display: block;
  background: var(--jp-border-color2);
  height: 100%;
  width: 5px;
}

.jp-mod-sideBySide.jp-Notebook
  .jp-CodeCell.jp-mod-resizedCell
  .jp-CellResizeHandle::after {
  background: var(--jp-border-color0);
}

.jp-CellResizeHandle {
  display: none;
}

/*-----------------------------------------------------------------------------
| Placeholder
|----------------------------------------------------------------------------*/

.jp-Cell-Placeholder {
  padding-left: 55px;
}

.jp-Cell-Placeholder-wrapper {
  background: #fff;
  border: 1px solid;
  border-color: #e5e6e9 #dfe0e4 #d0d1d5;
  border-radius: 4px;
  -webkit-border-radius: 4px;
  margin: 10px 15px;
}

.jp-Cell-Placeholder-wrapper-inner {
  padding: 15px;
  position: relative;
}

.jp-Cell-Placeholder-wrapper-body {
  background-repeat: repeat;
  background-size: 50% auto;
}

.jp-Cell-Placeholder-wrapper-body div {
  background: #f6f7f8;
  background-image: -webkit-linear-gradient(
    left,
    #f6f7f8 0%,
    #edeef1 20%,
    #f6f7f8 40%,
    #f6f7f8 100%
  );
  background-repeat: no-repeat;
  background-size: 800px 104px;
  height: 104px;
  position: absolute;
  right: 15px;
  left: 15px;
  top: 15px;
}

div.jp-Cell-Placeholder-h1 {
  top: 20px;
  height: 20px;
  left: 15px;
  width: 150px;
}

div.jp-Cell-Placeholder-h2 {
  left: 15px;
  top: 50px;
  height: 10px;
  width: 100px;
}

div.jp-Cell-Placeholder-content-1,
div.jp-Cell-Placeholder-content-2,
div.jp-Cell-Placeholder-content-3 {
  left: 15px;
  right: 15px;
  height: 10px;
}

div.jp-Cell-Placeholder-content-1 {
  top: 100px;
}

div.jp-Cell-Placeholder-content-2 {
  top: 120px;
}

div.jp-Cell-Placeholder-content-3 {
  top: 140px;
}

</style>
<style type="text/css">
/*-----------------------------------------------------------------------------
| Copyright (c) Jupyter Development Team.
| Distributed under the terms of the Modified BSD License.
|----------------------------------------------------------------------------*/

/*
The following CSS variables define the main, public API for styling JupyterLab.
These variables should be used by all plugins wherever possible. In other
words, plugins should not define custom colors, sizes, etc unless absolutely
necessary. This enables users to change the visual theme of JupyterLab
by changing these variables.

Many variables appear in an ordered sequence (0,1,2,3). These sequences
are designed to work well together, so for example, `--jp-border-color1` should
be used with `--jp-layout-color1`. The numbers have the following meanings:

* 0: super-primary, reserved for special emphasis
* 1: primary, most important under normal situations
* 2: secondary, next most important under normal situations
* 3: tertiary, next most important under normal situations

Throughout JupyterLab, we are mostly following principles from Google's
Material Design when selecting colors. We are not, however, following
all of MD as it is not optimized for dense, information rich UIs.
*/

:root {
  /* Elevation
   *
   * We style box-shadows using Material Design's idea of elevation. These particular numbers are taken from here:
   *
   * https://github.com/material-components/material-components-web
   * https://material-components-web.appspot.com/elevation.html
   */

  --jp-shadow-base-lightness: 0;
  --jp-shadow-umbra-color: rgba(
    var(--jp-shadow-base-lightness),
    var(--jp-shadow-base-lightness),
    var(--jp-shadow-base-lightness),
    0.2
  );
  --jp-shadow-penumbra-color: rgba(
    var(--jp-shadow-base-lightness),
    var(--jp-shadow-base-lightness),
    var(--jp-shadow-base-lightness),
    0.14
  );
  --jp-shadow-ambient-color: rgba(
    var(--jp-shadow-base-lightness),
    var(--jp-shadow-base-lightness),
    var(--jp-shadow-base-lightness),
    0.12
  );
  --jp-elevation-z0: none;
  --jp-elevation-z1: 0 2px 1px -1px var(--jp-shadow-umbra-color),
    0 1px 1px 0 var(--jp-shadow-penumbra-color),
    0 1px 3px 0 var(--jp-shadow-ambient-color);
  --jp-elevation-z2: 0 3px 1px -2px var(--jp-shadow-umbra-color),
    0 2px 2px 0 var(--jp-shadow-penumbra-color),
    0 1px 5px 0 var(--jp-shadow-ambient-color);
  --jp-elevation-z4: 0 2px 4px -1px var(--jp-shadow-umbra-color),
    0 4px 5px 0 var(--jp-shadow-penumbra-color),
    0 1px 10px 0 var(--jp-shadow-ambient-color);
  --jp-elevation-z6: 0 3px 5px -1px var(--jp-shadow-umbra-color),
    0 6px 10px 0 var(--jp-shadow-penumbra-color),
    0 1px 18px 0 var(--jp-shadow-ambient-color);
  --jp-elevation-z8: 0 5px 5px -3px var(--jp-shadow-umbra-color),
    0 8px 10px 1px var(--jp-shadow-penumbra-color),
    0 3px 14px 2px var(--jp-shadow-ambient-color);
  --jp-elevation-z12: 0 7px 8px -4px var(--jp-shadow-umbra-color),
    0 12px 17px 2px var(--jp-shadow-penumbra-color),
    0 5px 22px 4px var(--jp-shadow-ambient-color);
  --jp-elevation-z16: 0 8px 10px -5px var(--jp-shadow-umbra-color),
    0 16px 24px 2px var(--jp-shadow-penumbra-color),
    0 6px 30px 5px var(--jp-shadow-ambient-color);
  --jp-elevation-z20: 0 10px 13px -6px var(--jp-shadow-umbra-color),
    0 20px 31px 3px var(--jp-shadow-penumbra-color),
    0 8px 38px 7px var(--jp-shadow-ambient-color);
  --jp-elevation-z24: 0 11px 15px -7px var(--jp-shadow-umbra-color),
    0 24px 38px 3px var(--jp-shadow-penumbra-color),
    0 9px 46px 8px var(--jp-shadow-ambient-color);

  /* Borders
   *
   * The following variables, specify the visual styling of borders in JupyterLab.
   */

  --jp-border-width: 1px;
  --jp-border-color0: var(--md-grey-400);
  --jp-border-color1: var(--md-grey-400);
  --jp-border-color2: var(--md-grey-300);
  --jp-border-color3: var(--md-grey-200);
  --jp-inverse-border-color: var(--md-grey-600);
  --jp-border-radius: 2px;

  /* UI Fonts
   *
   * The UI font CSS variables are used for the typography all of the JupyterLab
   * user interface elements that are not directly user generated content.
   *
   * The font sizing here is done assuming that the body font size of --jp-ui-font-size1
   * is applied to a parent element. When children elements, such as headings, are sized
   * in em all things will be computed relative to that body size.
   */

  --jp-ui-font-scale-factor: 1.2;
  --jp-ui-font-size0: 0.83333em;
  --jp-ui-font-size1: 13px; /* Base font size */
  --jp-ui-font-size2: 1.2em;
  --jp-ui-font-size3: 1.44em;
  --jp-ui-font-family: system-ui, -apple-system, blinkmacsystemfont, 'Segoe UI',
    helvetica, arial, sans-serif, 'Apple Color Emoji', 'Segoe UI Emoji',
    'Segoe UI Symbol';

  /*
   * Use these font colors against the corresponding main layout colors.
   * In a light theme, these go from dark to light.
   */

  /* Defaults use Material Design specification */
  --jp-ui-font-color0: rgba(0, 0, 0, 1);
  --jp-ui-font-color1: rgba(0, 0, 0, 0.87);
  --jp-ui-font-color2: rgba(0, 0, 0, 0.54);
  --jp-ui-font-color3: rgba(0, 0, 0, 0.38);

  /*
   * Use these against the brand/accent/warn/error colors.
   * These will typically go from light to darker, in both a dark and light theme.
   */

  --jp-ui-inverse-font-color0: rgba(255, 255, 255, 1);
  --jp-ui-inverse-font-color1: rgba(255, 255, 255, 1);
  --jp-ui-inverse-font-color2: rgba(255, 255, 255, 0.7);
  --jp-ui-inverse-font-color3: rgba(255, 255, 255, 0.5);

  /* Content Fonts
   *
   * Content font variables are used for typography of user generated content.
   *
   * The font sizing here is done assuming that the body font size of --jp-content-font-size1
   * is applied to a parent element. When children elements, such as headings, are sized
   * in em all things will be computed relative to that body size.
   */

  --jp-content-line-height: 1.6;
  --jp-content-font-scale-factor: 1.2;
  --jp-content-font-size0: 0.83333em;
  --jp-content-font-size1: 14px; /* Base font size */
  --jp-content-font-size2: 1.2em;
  --jp-content-font-size3: 1.44em;
  --jp-content-font-size4: 1.728em;
  --jp-content-font-size5: 2.0736em;

  /* This gives a magnification of about 125% in presentation mode over normal. */
  --jp-content-presentation-font-size1: 17px;
  --jp-content-heading-line-height: 1;
  --jp-content-heading-margin-top: 1.2em;
  --jp-content-heading-margin-bottom: 0.8em;
  --jp-content-heading-font-weight: 500;

  /* Defaults use Material Design specification */
  --jp-content-font-color0: rgba(0, 0, 0, 1);
  --jp-content-font-color1: rgba(0, 0, 0, 0.87);
  --jp-content-font-color2: rgba(0, 0, 0, 0.54);
  --jp-content-font-color3: rgba(0, 0, 0, 0.38);
  --jp-content-link-color: var(--md-blue-900);
  --jp-content-font-family: system-ui, -apple-system, blinkmacsystemfont,
    'Segoe UI', helvetica, arial, sans-serif, 'Apple Color Emoji',
    'Segoe UI Emoji', 'Segoe UI Symbol';

  /*
   * Code Fonts
   *
   * Code font variables are used for typography of code and other monospaces content.
   */

  --jp-code-font-size: 13px;
  --jp-code-line-height: 1.3077; /* 17px for 13px base */
  --jp-code-padding: 5px; /* 5px for 13px base, codemirror highlighting needs integer px value */
  --jp-code-font-family-default: menlo, consolas, 'DejaVu Sans Mono', monospace;
  --jp-code-font-family: var(--jp-code-font-family-default);

  /* This gives a magnification of about 125% in presentation mode over normal. */
  --jp-code-presentation-font-size: 16px;

  /* may need to tweak cursor width if you change font size */
  --jp-code-cursor-width0: 1.4px;
  --jp-code-cursor-width1: 2px;
  --jp-code-cursor-width2: 4px;

  /* Layout
   *
   * The following are the main layout colors use in JupyterLab. In a light
   * theme these would go from light to dark.
   */

  --jp-layout-color0: white;
  --jp-layout-color1: white;
  --jp-layout-color2: var(--md-grey-200);
  --jp-layout-color3: var(--md-grey-400);
  --jp-layout-color4: var(--md-grey-600);

  /* Inverse Layout
   *
   * The following are the inverse layout colors use in JupyterLab. In a light
   * theme these would go from dark to light.
   */

  --jp-inverse-layout-color0: #111;
  --jp-inverse-layout-color1: var(--md-grey-900);
  --jp-inverse-layout-color2: var(--md-grey-800);
  --jp-inverse-layout-color3: var(--md-grey-700);
  --jp-inverse-layout-color4: var(--md-grey-600);

  /* Brand/accent */

  --jp-brand-color0: var(--md-blue-900);
  --jp-brand-color1: var(--md-blue-700);
  --jp-brand-color2: var(--md-blue-300);
  --jp-brand-color3: var(--md-blue-100);
  --jp-brand-color4: var(--md-blue-50);
  --jp-accent-color0: var(--md-green-900);
  --jp-accent-color1: var(--md-green-700);
  --jp-accent-color2: var(--md-green-300);
  --jp-accent-color3: var(--md-green-100);

  /* State colors (warn, error, success, info) */

  --jp-warn-color0: var(--md-orange-900);
  --jp-warn-color1: var(--md-orange-700);
  --jp-warn-color2: var(--md-orange-300);
  --jp-warn-color3: var(--md-orange-100);
  --jp-error-color0: var(--md-red-900);
  --jp-error-color1: var(--md-red-700);
  --jp-error-color2: var(--md-red-300);
  --jp-error-color3: var(--md-red-100);
  --jp-success-color0: var(--md-green-900);
  --jp-success-color1: var(--md-green-700);
  --jp-success-color2: var(--md-green-300);
  --jp-success-color3: var(--md-green-100);
  --jp-info-color0: var(--md-cyan-900);
  --jp-info-color1: var(--md-cyan-700);
  --jp-info-color2: var(--md-cyan-300);
  --jp-info-color3: var(--md-cyan-100);

  /* Cell specific styles */

  --jp-cell-padding: 5px;
  --jp-cell-collapser-width: 8px;
  --jp-cell-collapser-min-height: 20px;
  --jp-cell-collapser-not-active-hover-opacity: 0.6;
  --jp-cell-editor-background: var(--md-grey-100);
  --jp-cell-editor-border-color: var(--md-grey-300);
  --jp-cell-editor-box-shadow: inset 0 0 2px var(--md-blue-300);
  --jp-cell-editor-active-background: var(--jp-layout-color0);
  --jp-cell-editor-active-border-color: var(--jp-brand-color1);
  --jp-cell-prompt-width: 64px;
  --jp-cell-prompt-font-family: var(--jp-code-font-family-default);
  --jp-cell-prompt-letter-spacing: 0;
  --jp-cell-prompt-opacity: 1;
  --jp-cell-prompt-not-active-opacity: 0.5;
  --jp-cell-prompt-not-active-font-color: var(--md-grey-700);

  /* A custom blend of MD grey and blue 600
   * See https://meyerweb.com/eric/tools/color-blend/#546E7A:1E88E5:5:hex */
  --jp-cell-inprompt-font-color: #307fc1;

  /* A custom blend of MD grey and orange 600
   * https://meyerweb.com/eric/tools/color-blend/#546E7A:F4511E:5:hex */
  --jp-cell-outprompt-font-color: #bf5b3d;

  /* Notebook specific styles */

  --jp-notebook-padding: 10px;
  --jp-notebook-select-background: var(--jp-layout-color1);
  --jp-notebook-multiselected-color: var(--md-blue-50);

  /* The scroll padding is calculated to fill enough space at the bottom of the
  notebook to show one single-line cell (with appropriate padding) at the top
  when the notebook is scrolled all the way to the bottom. We also subtract one
  pixel so that no scrollbar appears if we have just one single-line cell in the
  notebook. This padding is to enable a 'scroll past end' feature in a notebook.
  */
  --jp-notebook-scroll-padding: calc(
    100% - var(--jp-code-font-size) * var(--jp-code-line-height) -
      var(--jp-code-padding) - var(--jp-cell-padding) - 1px
  );

  /* Rendermime styles */

  --jp-rendermime-error-background: #fdd;
  --jp-rendermime-table-row-background: var(--md-grey-100);
  --jp-rendermime-table-row-hover-background: var(--md-light-blue-50);

  /* Dialog specific styles */

  --jp-dialog-background: rgba(0, 0, 0, 0.25);

  /* Console specific styles */

  --jp-console-padding: 10px;

  /* Toolbar specific styles */

  --jp-toolbar-border-color: var(--jp-border-color1);
  --jp-toolbar-micro-height: 8px;
  --jp-toolbar-background: var(--jp-layout-color1);
  --jp-toolbar-box-shadow: 0 0 2px 0 rgba(0, 0, 0, 0.24);
  --jp-toolbar-header-margin: 4px 4px 0 4px;
  --jp-toolbar-active-background: var(--md-grey-300);

  /* Statusbar specific styles */

  --jp-statusbar-height: 24px;

  /* Input field styles */

  --jp-input-box-shadow: inset 0 0 2px var(--md-blue-300);
  --jp-input-active-background: var(--jp-layout-color1);
  --jp-input-hover-background: var(--jp-layout-color1);
  --jp-input-background: var(--md-grey-100);
  --jp-input-border-color: var(--jp-inverse-border-color);
  --jp-input-active-border-color: var(--jp-brand-color1);
  --jp-input-active-box-shadow-color: rgba(19, 124, 189, 0.3);

  /* General editor styles */

  --jp-editor-selected-background: #d9d9d9;
  --jp-editor-selected-focused-background: #d7d4f0;
  --jp-editor-cursor-color: var(--jp-ui-font-color0);

  /* Code mirror specific styles */

  --jp-mirror-editor-keyword-color: #008000;
  --jp-mirror-editor-atom-color: #88f;
  --jp-mirror-editor-number-color: #080;
  --jp-mirror-editor-def-color: #00f;
  --jp-mirror-editor-variable-color: var(--md-grey-900);
  --jp-mirror-editor-variable-2-color: rgb(0, 54, 109);
  --jp-mirror-editor-variable-3-color: #085;
  --jp-mirror-editor-punctuation-color: #05a;
  --jp-mirror-editor-property-color: #05a;
  --jp-mirror-editor-operator-color: #a2f;
  --jp-mirror-editor-comment-color: #408080;
  --jp-mirror-editor-string-color: #ba2121;
  --jp-mirror-editor-string-2-color: #708;
  --jp-mirror-editor-meta-color: #a2f;
  --jp-mirror-editor-qualifier-color: #555;
  --jp-mirror-editor-builtin-color: #008000;
  --jp-mirror-editor-bracket-color: #997;
  --jp-mirror-editor-tag-color: #170;
  --jp-mirror-editor-attribute-color: #00c;
  --jp-mirror-editor-header-color: blue;
  --jp-mirror-editor-quote-color: #090;
  --jp-mirror-editor-link-color: #00c;
  --jp-mirror-editor-error-color: #f00;
  --jp-mirror-editor-hr-color: #999;

  /*
    RTC user specific colors.
    These colors are used for the cursor, username in the editor,
    and the icon of the user.
  */

  --jp-collaborator-color1: #ffad8e;
  --jp-collaborator-color2: #dac83d;
  --jp-collaborator-color3: #72dd76;
  --jp-collaborator-color4: #00e4d0;
  --jp-collaborator-color5: #45d4ff;
  --jp-collaborator-color6: #e2b1ff;
  --jp-collaborator-color7: #ff9de6;

  /* Vega extension styles */

  --jp-vega-background: white;

  /* Sidebar-related styles */

  --jp-sidebar-min-width: 250px;

  /* Search-related styles */

  --jp-search-toggle-off-opacity: 0.5;
  --jp-search-toggle-hover-opacity: 0.8;
  --jp-search-toggle-on-opacity: 1;
  --jp-search-selected-match-background-color: rgb(245, 200, 0);
  --jp-search-selected-match-color: black;
  --jp-search-unselected-match-background-color: var(
    --jp-inverse-layout-color0
  );
  --jp-search-unselected-match-color: var(--jp-ui-inverse-font-color0);

  /* Icon colors that work well with light or dark backgrounds */
  --jp-icon-contrast-color0: var(--md-purple-600);
  --jp-icon-contrast-color1: var(--md-green-600);
  --jp-icon-contrast-color2: var(--md-pink-600);
  --jp-icon-contrast-color3: var(--md-blue-600);

  /* Button colors */
  --jp-accept-color-normal: var(--md-blue-700);
  --jp-accept-color-hover: var(--md-blue-800);
  --jp-accept-color-active: var(--md-blue-900);
  --jp-warn-color-normal: var(--md-red-700);
  --jp-warn-color-hover: var(--md-red-800);
  --jp-warn-color-active: var(--md-red-900);
  --jp-reject-color-normal: var(--md-grey-600);
  --jp-reject-color-hover: var(--md-grey-700);
  --jp-reject-color-active: var(--md-grey-800);

  /* File or activity icons and switch semantic variables */
  --jp-jupyter-icon-color: #f37626;
  --jp-notebook-icon-color: #f37626;
  --jp-json-icon-color: var(--md-orange-700);
  --jp-console-icon-background-color: var(--md-blue-700);
  --jp-console-icon-color: white;
  --jp-terminal-icon-background-color: var(--md-grey-800);
  --jp-terminal-icon-color: var(--md-grey-200);
  --jp-text-editor-icon-color: var(--md-grey-700);
  --jp-inspector-icon-color: var(--md-grey-700);
  --jp-switch-color: var(--md-grey-400);
  --jp-switch-true-position-color: var(--md-orange-900);
}
</style>
<style type="text/css">
/* Force rendering true colors when outputing to pdf */
* {
  -webkit-print-color-adjust: exact;
}

/* Misc */
a.anchor-link {
  display: none;
}

/* Input area styling */
.jp-InputArea {
  overflow: hidden;
}

.jp-InputArea-editor {
  overflow: hidden;
}

.cm-editor.cm-s-jupyter .highlight pre {
/* weird, but --jp-code-padding defined to be 5px but 4px horizontal padding is hardcoded for pre.cm-line */
  padding: var(--jp-code-padding) 4px;
  margin: 0;

  font-family: inherit;
  font-size: inherit;
  line-height: inherit;
  color: inherit;

}

.jp-OutputArea-output pre {
  line-height: inherit;
  font-family: inherit;
}

.jp-RenderedText pre {
  color: var(--jp-content-font-color1);
  font-size: var(--jp-code-font-size);
}

/* Hiding the collapser by default */
.jp-Collapser {
  display: none;
}

@page {
    margin: 0.5in; /* Margin for each printed piece of paper */
}

@media print {
  .jp-Cell-inputWrapper,
  .jp-Cell-outputWrapper {
    display: block;
  }
}
</style>
<!-- Load mathjax -->
<script src="https://cdnjs.cloudflare.com/ajax/libs/mathjax/2.7.7/latest.js?config=TeX-AMS_CHTML-full,Safe"> </script>
<!-- MathJax configuration -->
<script type="text/x-mathjax-config">
    init_mathjax = function() {
        if (window.MathJax) {
        // MathJax loaded
            MathJax.Hub.Config({
                TeX: {
                    equationNumbers: {
                    autoNumber: "AMS",
                    useLabelIds: true
                    }
                },
                tex2jax: {
                    inlineMath: [ ['$','$'], ["\\(","\\)"] ],
                    displayMath: [ ['$$','$$'], ["\\[","\\]"] ],
                    processEscapes: true,
                    processEnvironments: true
                },
                displayAlign: 'center',
                CommonHTML: {
                    linebreaks: {
                    automatic: true
                    }
                }
            });

            MathJax.Hub.Queue(["Typeset", MathJax.Hub]);
        }
    }
    init_mathjax();
    </script>
<!-- End of mathjax configuration --><script type="module">
  document.addEventListener("DOMContentLoaded", async () => {
    const diagrams = document.querySelectorAll(".jp-Mermaid > pre.mermaid");
    // do not load mermaidjs if not needed
    if (!diagrams.length) {
      return;
    }
    const mermaid = (await import("https://cdnjs.cloudflare.com/ajax/libs/mermaid/10.6.0/mermaid.esm.min.mjs")).default;
    const parser = new DOMParser();

    mermaid.initialize({
      maxTextSize: 100000,
      startOnLoad: false,
      fontFamily: window
        .getComputedStyle(document.body)
        .getPropertyValue("--jp-ui-font-family"),
      theme: document.querySelector("body[data-jp-theme-light='true']")
        ? "default"
        : "dark",
    });

    let _nextMermaidId = 0;

    function makeMermaidImage(svg) {
      const img = document.createElement("img");
      const doc = parser.parseFromString(svg, "image/svg+xml");
      const svgEl = doc.querySelector("svg");
      const { maxWidth } = svgEl?.style || {};
      const firstTitle = doc.querySelector("title");
      const firstDesc = doc.querySelector("desc");

      img.setAttribute("src", `data:image/svg+xml,${encodeURIComponent(svg)}`);
      if (maxWidth) {
        img.width = parseInt(maxWidth);
      }
      if (firstTitle) {
        img.setAttribute("alt", firstTitle.textContent);
      }
      if (firstDesc) {
        const caption = document.createElement("figcaption");
        caption.className = "sr-only";
        caption.textContent = firstDesc.textContent;
        return [img, caption];
      }
      return [img];
    }

    async function makeMermaidError(text) {
      let errorMessage = "";
      try {
        await mermaid.parse(text);
      } catch (err) {
        errorMessage = `${err}`;
      }

      const result = document.createElement("details");
      result.className = 'jp-RenderedMermaid-Details';
      const summary = document.createElement("summary");
      summary.className = 'jp-RenderedMermaid-Summary';
      const pre = document.createElement("pre");
      const code = document.createElement("code");
      code.innerText = text;
      pre.appendChild(code);
      summary.appendChild(pre);
      result.appendChild(summary);

      const warning = document.createElement("pre");
      warning.innerText = errorMessage;
      result.appendChild(warning);
      return [result];
    }

    async function renderOneMarmaid(src) {
      const id = `jp-mermaid-${_nextMermaidId++}`;
      const parent = src.parentNode;
      let raw = src.textContent.trim();
      const el = document.createElement("div");
      el.style.visibility = "hidden";
      document.body.appendChild(el);
      let results = null;
      let output = null;
      try {
        const { svg } = await mermaid.render(id, raw, el);
        results = makeMermaidImage(svg);
        output = document.createElement("figure");
        results.map(output.appendChild, output);
      } catch (err) {
        parent.classList.add("jp-mod-warning");
        results = await makeMermaidError(raw);
        output = results[0];
      } finally {
        el.remove();
      }
      parent.classList.add("jp-RenderedMermaid");
      parent.appendChild(output);
    }

    void Promise.all([...diagrams].map(renderOneMarmaid));
  });
</script>
<style>
  .jp-Mermaid:not(.jp-RenderedMermaid) {
    display: none;
  }

  .jp-RenderedMermaid {
    overflow: auto;
    display: flex;
  }

  .jp-RenderedMermaid.jp-mod-warning {
    width: auto;
    padding: 0.5em;
    margin-top: 0.5em;
    border: var(--jp-border-width) solid var(--jp-warn-color2);
    border-radius: var(--jp-border-radius);
    color: var(--jp-ui-font-color1);
    font-size: var(--jp-ui-font-size1);
    white-space: pre-wrap;
    word-wrap: break-word;
  }

  .jp-RenderedMermaid figure {
    margin: 0;
    overflow: auto;
    max-width: 100%;
  }

  .jp-RenderedMermaid img {
    max-width: 100%;
  }

  .jp-RenderedMermaid-Details > pre {
    margin-top: 1em;
  }

  .jp-RenderedMermaid-Summary {
    color: var(--jp-warn-color2);
  }

  .jp-RenderedMermaid:not(.jp-mod-warning) pre {
    display: none;
  }

  .jp-RenderedMermaid-Summary > pre {
    display: inline-block;
    white-space: normal;
  }
</style>
<!-- End of mermaid configuration --></head>
<body class="jp-Notebook" data-jp-theme-light="true" data-jp-theme-name="JupyterLab Light">
<main><div class="jp-Cell jp-CodeCell jp-Notebook-cell jp-mod-noOutputs">
<div class="jp-Cell-inputWrapper" tabindex="0">
<div class="jp-Collapser jp-InputCollapser jp-Cell-inputCollapser">
</div>
<div class="jp-InputArea jp-Cell-inputArea">
<div class="jp-InputPrompt jp-InputArea-prompt">In [14]:</div>
<div class="jp-CodeMirrorEditor jp-Editor jp-InputArea-editor" data-type="inline">
<div class="cm-editor cm-s-jupyter">
<div class="highlight hl-ipython3"><pre><span></span><span class="kn">import</span> <span class="nn">networkx</span> <span class="k">as</span> <span class="nn">nx</span>
<span class="kn">import</span> <span class="nn">matplotlib.pyplot</span> <span class="k">as</span> <span class="nn">plt</span>
</pre></div>
</div>
</div>
</div>
</div>
</div><div class="jp-Cell jp-CodeCell jp-Notebook-cell">
<div class="jp-Cell-inputWrapper" tabindex="0">
<div class="jp-Collapser jp-InputCollapser jp-Cell-inputCollapser">
</div>
<div class="jp-InputArea jp-Cell-inputArea">
<div class="jp-InputPrompt jp-InputArea-prompt">In [15]:</div>
<div class="jp-CodeMirrorEditor jp-Editor jp-InputArea-editor" data-type="inline">
<div class="cm-editor cm-s-jupyter">
<div class="highlight hl-ipython3"><pre><span></span><span class="c1"># _cracks knuckles_ let's make a random graph</span>
<span class="n">G</span> <span class="o">=</span> <span class="n">nx</span><span class="o">.</span><span class="n">fast_gnp_random_graph</span><span class="p">(</span><span class="mi">10</span><span class="p">,</span> <span class="mf">.2</span><span class="p">)</span>
<span class="n">nx</span><span class="o">.</span><span class="n">draw</span><span class="p">(</span><span class="n">G</span><span class="p">)</span>
</pre></div>
</div>
</div>
</div>
</div>
<div class="jp-Cell-outputWrapper">
<div class="jp-Collapser jp-OutputCollapser jp-Cell-outputCollapser">
</div>
<div class="jp-OutputArea jp-Cell-outputArea">
<div class="jp-OutputArea-child">
<div class="jp-OutputPrompt jp-OutputArea-prompt"></div>
<div class="jp-RenderedImage jp-OutputArea-output" tabindex="0">
<img alt="No description has been provided for this image" class="" src="data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAApQAAAHzCAYAAACe1o1DAAAAOXRFWHRTb2Z0d2FyZQBNYXRwbG90bGliIHZlcnNpb24zLjguMSwgaHR0cHM6Ly9tYXRwbG90bGliLm9yZy/SrBM8AAAACXBIWXMAAA9hAAAPYQGoP6dpAABrOUlEQVR4nO3dd1xU174F8DXDwCCIBRRsEFSKiohjicbEGBJj2r0mJlgimosNjRgr9thLYsSO5WKvsUWj8WpsIdaIGhuiUmygRpEqggLDnPeHgaexwsywZ86s7+eTz3s34DnLKLL8nbP3VkiSJIGIiIiIqISUogMQERERkXljoSQiIiIivbBQEhEREZFeWCiJiIiISC8slERERESkFxZKIiIiItILCyURERER6YWFkoiIiIj0wkJJRERERHphoSQiIiIivbBQEhEREZFeWCiJiIiISC8slERERESkFxZKIiIiItILCyURERER6YWFkoiIiIj0wkJJRERERHphoSQiIiIivbBQEhEREZFeWCiJiIiISC8slERERESkFxZKIiIiItILCyURERER6YWFkoiIiIj0wkJJRERERHphoSQiIiIivbBQEhEREZFeWCiJiIiISC8slERERESkFxZKIiIiItILCyURERER6YWFkoiIiIj0wkJJRERERHphoSQiIiIivbBQEhEREZFeWCiJiIiISC8slERERESkF5XoAERERCWRnavFtdRs5Gl1sFEp4e5kD3s1v60RicCvPCIiMhvxd7KwNioRkbHJSEzLgfTYxxQA3Bzt4O/tjMBmbvB0cRAVk8jiKCRJkl7+aUREROIkpeVg1NZoHEpIgZVSgQLd8791FX68pUclTG3nC1dHu1JMSmSZWCiJiMikrT+RiHHbY6DVSS8skv9kpVRApVRgQlsfdGrqZsSERMRCSUREJis8Mh5he+L0vk5oGy/08/c0QCIiehau8iYiIpO0/kSiQcokAITticOGE4kGuRYRPY2FkoiITE5SWg7GbY8x6DXHbo9BUlqOQa9JRI+wUBIRkckZtTUa2mK8L/kqtDoJo7ZGG/SaRPQICyUREZmU+DtZOJSQUqwFOK+iQCfhUEIKEpKzDHpdImKhJCIiE7M2KhFWSoVRrm2lVGDNMb5LSWRoLJRERGRSImOTDT6dLFSgkxAZl2yUaxNZMhZKIiIyGfdztUg08sKZ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GRMLJRERUTEENnMz6j6UXZq7GeXaRMbEQklERFQMni4OaOlRyeBTSiulAi09KsHD2cGg1yUqDSyURERExTS1nS9UBi6UKqUCU9v5GvSaRKWFhZKIiKiYXB3tMKGtj0GvObGtD1yNvOCHyFhYKImIiEqgU1M3hLbxMsi1hrbxRsemfHeSzJdCkiRjnyJFREQkW+tPJGLc9hhodVKxFutYKRVQKRWY2NaHZZLMHgslERGRnpLScjBqazQOJaTASql4YbEs/HhLj0qY2s6Xj7lJFlgoiYiIDCT+ThbWRiUiMi4Ziak5ePwbrAKPNi3393JGl+ZuXM1NssJCSUREZATZuVpcS81GnlYHG5US7k72PAGHZIuFkoiIiIj0wlXeRERERKQXFkoiIiIi0gsLJRERERHphYWSiIiIiPTCQklEREREemGhJCIiIiK9sFASERERkV5YKImIiIhILyyURERERKQXFkoiIiIi0gsLJRERERHphYWSiIiIiPTCQklEREREemGhJCIiIiK9sFASERERkV5YKImIiIhILyyURERERKQXFkoiIiIi0gsLJRERERHphYWSiIiIiPTCQklEREREemGhJCIiIiK9sFASERERkV5YKImIiIhILyyURERERKQXFkoiIiIi0gsLJRERERHphYWSiIiIiPTCQklEREREemGhJCIiIiK9sFASERERkV5YKImIiIhILyyURERERKQXFkoiIiIi0gsLJRERERHphYWSiIiIiPTCQklEREREevk/mNB6zkrXWZwAAAAASUVORK5CYII="/>
</div>
</div>
</div>
</div>
</div><div class="jp-Cell jp-CodeCell jp-Notebook-cell jp-mod-noOutputs">
<div class="jp-Cell-inputWrapper" tabindex="0">
<div class="jp-Collapser jp-InputCollapser jp-Cell-inputCollapser">
</div>
<div class="jp-InputArea jp-Cell-inputArea">
<div class="jp-InputPrompt jp-InputArea-prompt">In [18]:</div>
<div class="jp-CodeMirrorEditor jp-Editor jp-InputArea-editor" data-type="inline">
<div class="cm-editor cm-s-jupyter">
<div class="highlight hl-ipython3"><pre><span></span><span class="c1"># let's build a simple directed graph from scratch</span>
<span class="n">G</span> <span class="o">=</span> <span class="n">nx</span><span class="o">.</span><span class="n">DiGraph</span><span class="p">()</span>

<span class="c1"># nodes named "1" and "2" are implicitly created by adding edges between nodes</span>
<span class="c1"># with those names</span>
<span class="n">G</span><span class="o">.</span><span class="n">add_edge</span><span class="p">(</span><span class="mi">1</span><span class="p">,</span> <span class="mi">2</span><span class="p">)</span>

<span class="c1"># weight acts like a cost function, where higher values mean more cost</span>
<span class="n">G</span><span class="o">.</span><span class="n">add_edge</span><span class="p">(</span><span class="mi">2</span><span class="p">,</span> <span class="mi">3</span><span class="p">,</span> <span class="n">weight</span><span class="o">=</span><span class="mf">0.5</span><span class="p">)</span>

<span class="c1"># we can pass in any python object, even function objects</span>
<span class="n">G</span><span class="o">.</span><span class="n">add_edge</span><span class="p">(</span><span class="mi">1</span><span class="p">,</span> <span class="s2">"four"</span><span class="p">)</span>
<span class="n">G</span><span class="o">.</span><span class="n">add_edge</span><span class="p">(</span><span class="mi">2</span><span class="p">,</span> <span class="nb">print</span><span class="p">)</span>
<span class="n">G</span><span class="o">.</span><span class="n">add_edge</span><span class="p">(</span><span class="mi">3</span><span class="p">,</span> <span class="mi">3</span><span class="p">)</span>

<span class="c1"># come on, copying and pasting code is for losers</span>
<span class="n">G</span><span class="o">.</span><span class="n">add_edges_from</span><span class="p">([(</span><span class="n">n</span><span class="p">,</span> <span class="n">n</span><span class="o">+</span><span class="mi">1</span><span class="p">)</span> <span class="k">for</span> <span class="n">n</span> <span class="ow">in</span> <span class="nb">range</span><span class="p">(</span><span class="mi">10</span><span class="p">,</span> <span class="mi">20</span><span class="p">,</span> <span class="mi">2</span><span class="p">)])</span>
</pre></div>
</div>
</div>
</div>
</div>
</div><div class="jp-Cell jp-CodeCell jp-Notebook-cell">
<div class="jp-Cell-inputWrapper" tabindex="0">
<div class="jp-Collapser jp-InputCollapser jp-Cell-inputCollapser">
</div>
<div class="jp-InputArea jp-Cell-inputArea">
<div class="jp-InputPrompt jp-InputArea-prompt">In [19]:</div>
<div class="jp-CodeMirrorEditor jp-Editor jp-InputArea-editor" data-type="inline">
<div class="cm-editor cm-s-jupyter">
<div class="highlight hl-ipython3"><pre><span></span><span class="c1"># draw the graph (with labels) a few different ways</span>
<span class="n">nx</span><span class="o">.</span><span class="n">draw</span><span class="p">(</span><span class="n">G</span><span class="p">,</span> <span class="n">with_labels</span><span class="o">=</span><span class="kc">True</span><span class="p">)</span>
<span class="n">plt</span><span class="o">.</span><span class="n">show</span><span class="p">()</span>

<span class="n">nx</span><span class="o">.</span><span class="n">draw_spring</span><span class="p">(</span><span class="n">G</span><span class="p">,</span> <span class="n">with_labels</span><span class="o">=</span><span class="kc">True</span><span class="p">)</span>
<span class="n">plt</span><span class="o">.</span><span class="n">show</span><span class="p">()</span>
</pre></div>
</div>
</div>
</div>
</div>
<div class="jp-Cell-outputWrapper">
<div class="jp-Collapser jp-OutputCollapser jp-Cell-outputCollapser">
</div>
<div class="jp-OutputArea jp-Cell-outputArea">
<div class="jp-OutputArea-child">
<div class="jp-OutputPrompt jp-OutputArea-prompt"></div>
<div class="jp-RenderedImage jp-OutputArea-output" tabindex="0">
<img alt="No description has been provided for this image" class="" src="data:image/png;base64,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Fda97xF3FyCipvivAVEPWpQSizA/JV7YlgujWWozXV5rZKIAuShgVWo8g0nqkGV3gdLt6yEZdFBGjobMPxQmTSW0x3bDWH4OwdcvgKjwAQAYq0uhOfoDAEBfXAAA1p0F5IHh8Bt9vbVs7i5ARM0JkiR1vkUjIocoqtB1Kg8zAOv5yTFhWD0zgcPc1Cn5JbW48Y090B7LgObw9zBcKoS5rhaiwgeK/jHwv2qGTS7v+rOHUfLp8lbLUkaORv/Zr9gc27n4Oi4IIyIrBpRELpRfUovN2Wqk55VCXa6z2epFQOOwYkpcOOZMjGLjTV029/1s6+4CjmLZXWDjgqSOLyaiPoMBJVEv0TQPs0IuIjpUxTlqZJeiCh2mrc2A3oHb+yjlInYunsKeciKywYCSiMiDpeWosWzLEYeV9+qdCZzDS0QtcJU3EZEH4+4CRNQT2ENJRNQHpOWoubsAETkNA0oioj6CuwsQkbMwoCQi6mO4uwARORoDSiKiPoy7CxCRIzCgJCIiIiK7cJU3EREREdmFASURERER2YUBJRERERHZhQElEREREdmFASURERER2YUBJRERERHZhQElEREREdmFASURERER2YUBJRERERHZhQElEREREdmFASURERER2YUBJRERERHZhQElEREREdmFASURERER2YUBJRERERHZhQElEREREdmFASURERER2YUBJRERERHZhQElEREREdmFASURERER2YUBJRERERHZhQElEREREdmFASURERER2YUBJRERERHZhQElEREREdmFASURERER2YUBJRERERHZhQElEREREdmFASURERER2YUBJRERERHZhQElEREREdmFASURERER2YUBJRERERHZhQElEREREdmFASURERER2YUBJRERERHZhQElEREREdnl/wMgOuxfsZ2O9AAAAABJRU5ErkJggg=="/>
</div>
</div>
<div class="jp-OutputArea-child">
<div class="jp-OutputPrompt jp-OutputArea-prompt"></div>
<div class="jp-RenderedImage jp-OutputArea-output" tabindex="0">
<img alt="No description has been provided for this image" class="" src="data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAApQAAAHzCAYAAACe1o1DAAAAOXRFWHRTb2Z0d2FyZQBNYXRwbG90bGliIHZlcnNpb24zLjguMSwgaHR0cHM6Ly9tYXRwbG90bGliLm9yZy/SrBM8AAAACXBIWXMAAA9hAAAPYQGoP6dpAACM6ElEQVR4nOzdeVxU9foH8M+ZGRj2XRAVxF1DXDMUWSK3zDK1MnNXaLtZN0u7Zd1SK9OyrFu/bvcmqaFpZVq2XrNccEPS3HADlUURkB1mYIaZOb8/kMmRHWY4zPB5v1697mXOOd/zDALznO/yfAVRFEUQERERETWTTOoAiIiIiMi6MaEkIiIiohZhQklERERELcKEkoiIiIhahAklEREREbUIE0oiIiIiahEmlERERETUIkwoiYiIiKhFmFASERERUYswoSQiIiKiFmFCSUREREQtwoSSiIiIiFqECSURERERtQgTSiIiIiJqESaURERERNQiTCiJiIiIqEWYUBIRERFRizChJCIiIqIWYUJJRERERC3ChJKIiIiIWoQJJRERERG1CBNKIiIiImoRJpRERERE1CJMKImIiIioRZhQEhEREVGLMKEkIiIiohZhQklERERELcKEkoioDRBFETNmzMCbb76JkpISqcMhImoSQRRFUeogiIjaO61WC6VSCQBwc3PDCy+8gKeffhpubm4SR0ZE1DAmlEREbcDNCSUAyGQyuLi4YNGiRcbE0mAwQKFQSBglEVHtmFASEbUBaWlp6NatW63H3N3dMXToUPz++++QyWTw8PBASEgIBg4ciEGDBmHQoEEYMGAA5HJ5K0dNRFSFCSURkUQMBgN27dqFf/3rX/jxxx9NjgmCAFEUERAQgFdffRVDhw7FiRMnUFFRgby8PJw8eRLHjx9HamoqRFFEYGAgHn30UcTExMDf31+id0RE7RUTSiKiViaKIjZv3ozXX38d586dw6BBg/DEE0/giSeeMCaSAwcOxBtvvIEJEyZAEIQ62yorK8PRo0fx+eefY/PmzaisrMT999+PN998E3369GnFd0VE7RkTSiKiVlRcXIwnn3wSmzdvxv3334/nn38e4eHhMBgM8PHxQdeuXRuVSNamqKgIGzduxJo1a5CVlYW33noLzzzzDGQyFvQgIstiQklE1EqOHDmChx9+GAUFBfjkk0/wyCOPmBwvLy+Hg4ND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"/>
</div>
</div>
</div>
</div>
</div>
</main>
</body>
</html>
