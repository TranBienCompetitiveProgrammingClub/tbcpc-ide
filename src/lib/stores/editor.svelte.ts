import { getUITheme, getEditorTheme } from "$lib/theme";

type Tab = {
  id: string;
  label: string;
  content: string;
  ioMode: "stdio" | "file";
  filePath: string | null;
};

let tabs = $state<Tab[]>([]);
let activeId = $state<string | null>(null);
let fontSize = $state(14);
let isDark = $state(true);
let inputBuffer = $state("");
let outputBuffer = $state("");
let cppVersion = $state("14");

export const editorStore = {
  get tabs() {
    return tabs;
  },
  get activeId() {
    return activeId;
  },
  get fontSize() {
    return fontSize;
  },
  get isDark() {
    return isDark;
  },
  get editorTheme() {
    return getEditorTheme(isDark);
  },
  get uiTheme() {
    return getUITheme(isDark);
  },
  get inputBuffer() {
    return inputBuffer;
  },
  get outputBuffer() {
    return outputBuffer;
  },
  get cppVersion() {
    return cppVersion;
  },
  setCppVersion(v: string) {
    cppVersion = v;
  },

  addTab(label: string, content: string = "", filePath: string | null = null) {
    const id = crypto.randomUUID();
    tabs.push({ id, label, content, ioMode: "stdio", filePath });
    activeId = id;
  },

  removeTab(id: string) {
    const idx = tabs.findIndex((t) => t.id === id);
    tabs = tabs.filter((t) => t.id !== id);
    if (activeId === id) {
      activeId = tabs[Math.min(idx, tabs.length - 1)]?.id ?? null;
    }
  },

  setActive(id: string) {
    activeId = id;
  },
  setFontSize(v: number) {
    fontSize = v;
  },
  setInputBuffer(v: string) {
    inputBuffer = v;
  },
  setOutputBuffer(v: string) {
    outputBuffer = v;
  },
  setFilePath(id: string, path: string) {
    const tab = tabs.find((t) => t.id === id);
    if (tab) tab.filePath = path;
  },
  toggleTheme() {
    isDark = !isDark;
    document.documentElement.setAttribute("data-theme", getUITheme(isDark));
  },
};
