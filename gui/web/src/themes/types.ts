import { GlobalThemeOverrides } from "naive-ui";

export interface ThemeVars extends GlobalThemeOverrides {
  custom?: {
    assistantMsgBgColor: string;
    assistantMsgColor: string;
    userMsgBgColor: string;
    userMsgColor: string;
    activeMenuBgColor: string;
    explorerBgColor: string;
    inputMsgColor: string;
    explorerColor: string;
    explorerActiveBgColor: string;
    explorerActiveColor: string;
  };
}