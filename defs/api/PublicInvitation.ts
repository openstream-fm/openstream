// This file was generated by [ts-rs](https://github.com/Aleph-Alpha/ts-rs). Do not edit this file manually.
import type { AccountInvitationState } from "../db/AccountInvitationState";
import type { DateTime } from "../DateTime";
import type { InvitationAccount } from "./InvitationAccount";
import type { InvitationAdminSender } from "./InvitationAdminSender";
import type { InvitationReceiver } from "./InvitationReceiver";
import type { InvitationUserSender } from "./InvitationUserSender";

export type PublicInvitation = {
  id: string;
  user_sender_id: string | null;
  admin_sender_id: string | null;
  account_id: string;
  receiver_email: string;
  created_at: DateTime;
  is_expired: boolean;
  account: InvitationAccount | null;
  user_sender: InvitationUserSender | null;
  admin_sender: InvitationAdminSender | null;
  receiver: InvitationReceiver | null;
} & AccountInvitationState;
