/// file: misc.zh.ts
/// file: wip.zh.ts
const locale: typeof import("./misc.en").default = {
  Relay: "中继",
  Settings_updated: "设置已更新",
  Master_relay: "主中继",
  Enable_master_relay: "启用主中继",
  Master_Relay_URL: "主中继URL",
  Save: "保存",
  delete_station_not_owner_message_html: "只有账户管理员才能删除电台。<br/><br/>如果你想删除这个电台，请联系账户管理员。",

  Cancel: "取消",
  OK: "确定",
  Transfer_station: "转移电台",
  Station_name: "电台名称",
  station_transfer_title: "将电台 @station 转移到你的另一个账户",
  station_transfer_message_html: "要将电台 @station 转移到你的另一个账户，请输入电台名称：<b>@station</b>，然后选择目标账户。",
  station_transfer_not_owner_message_html: "只有账户管理员才能在账户之间转移电台。<br/><br/>请联系账户管理员以将电台转移到另一个账户。",
  station_transfer_no_targets_message: "你必须有另一个账户的访问权限，才能将此电台转移到另一个账户。",
  Station_name_do_not_match: "电台名称不匹配",
  Target_account_is_required: "需要目标账户",
  Station_transferred: "电台已转移",
  Select_a_target_account: "选择一个目标账户",

  Welcome: "欢迎",

  account_welcome_title_html: "你好，<b>@name</b>",
  account_welcome_message_1_html: "欢迎来到 <b>@brand</b>",
  account_welcome_message_2_html: "从现在开始，你是你新账户的所有者",
  account_welcome_message_3_html: "要开始现在的广播，将你的第一个电台添加到你的账户",

  Create_my_first_station: "创建我的第一个电台",

  "0_listeners": "0位听众",
  "1_listener": "1位听众",
  "n_listeners": "@n位听众",
}

export default locale;