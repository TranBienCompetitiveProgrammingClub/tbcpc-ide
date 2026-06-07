import { catppuccinMocha } from "@catppuccin/codemirror";
import { githubLight } from "@uiw/codemirror-theme-github";
import { githubDark } from "@uiw/codemirror-theme-github";
import type { Extension } from "@codemirror/state";

export const DARK_UI = "mocha";
export const LIGHT_UI = "latte";

export const DARK_EDITOR: Extension = githubDark;
export const LIGHT_EDITOR: Extension = githubLight;

export function getEditorTheme(isDark: boolean): Extension {
  return isDark ? DARK_EDITOR : LIGHT_EDITOR;
}

export function getUITheme(isDark: boolean): string {
  return isDark ? DARK_UI : LIGHT_UI;
}
