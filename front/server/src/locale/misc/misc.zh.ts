const locale: typeof import("./misc.en").default = {
  Relay: "中继",
  Settings_updated: "设置已更新",
  Master_relay: "主中继",
  Enable_master_relay: "启用主中继",
  Master_Relay_URL: "主中继URL",
  Save: "保存",
  delete_station_not_owner_message_html: "只有账户管理员可以删除电台。<br/><br/>如果你想删除这个电台，请联系账户管理员。",

  Cancel: "取消",
  OK: "确定",
  Delete: "删除",
  Create: "创建",
  Copy: "复制",
  Done: "完成",

  Copied_to_clipboard: "已复制到剪贴板",

  Id: "编号",
  Title: "标题",
  Created: "已创建",
  Last_used: "最后使用",

  Transfer_station: "转移电台",
  Station_name: "电台名称",
  station_transfer_title: "将电台 @station 转移到你的另一个账户",
  station_transfer_message_html: "要将电台 @station 转移到另一个账户，请输入电台名称：<b>@station</b> 并选择目标账户。",
  station_transfer_not_owner_message_html: "只有账户管理员可以在账户之间转移电台。<br/><br/>请联系账户管理员以将电台转移到另一个账户。",
  station_transfer_no_targets_message: "你必须有权访问另一个账户才能将该电台转移到另一个账户。",
  Station_name_do_not_match: "电台名称不匹配",
  Target_account_is_required: "需要目标账户",
  Station_transferred: "电台已转移",
  Select_a_target_account: "选择一个目标账户",

  Type_password_proceed: "输入你的密码以继续此操作。",

  Welcome: "欢迎",

  account_welcome_title_html: "你好 <b>@name</b>",
  account_welcome_message_1_html: "欢迎来到 <b>@brand</b>",
  account_welcome_message_2_html: "从现在起，你是你的新账户的拥有者",
  account_welcome_message_3_html: "要开始现在就播放，请将你的第一个电台添加到你的账户",

  Create_my_first_station: "创建我的第一个电台",

  Your_email: "你的邮箱",
  Your_password: "你的密码",

  "0_listeners": "0 名听众",
  "1_listener": "1 名听众",
  "n_listeners": "@n 名听众",

  Enable_master_relay_redirect_mode: "启用主中继重定向模式",
  External_relay_error: "主中继错误",

  player: {
    Relay: "中继",
    Live_Streaming: "直播",
    Playlist: "播放列表",
  },

  This_action_is_permanent: "此操作是永久性的。",
  
  api_keys: {
    API_Keys: "API密钥",
    API_key_deleted: "API密钥已删除",
    API_keys_page_message: "创建API密钥以编程方式访问你的账户或授权第三方应用和服务。",
		Create_a_new_API_key: "创建新的API密钥",
    Remove_API_key: "删除API密钥",
    API_key_title: "API密钥标题",
    API_key_title_explain: "此标题将被用来帮助你识别这个API密钥",
    Copy_contents_message: "复制API密钥内容。此代码不会再次显示。",
    API_key_contents: "API密钥内容",
  }
}

export default locale;