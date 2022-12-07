import { BaseAudioUploadOperation } from "../defs/db/BaseAudioUploadOperation";
import { AudioUploadOperationState } from "../defs/db/AudioUploadOperationState";

export type AudioUploadOperation = BaseAudioUploadOperation & AudioUploadOperationState;