"use client";
import { motion } from "motion/react";
import { useCharacter } from "@/providers/CharacterProvider";
import { Button } from "@/components/ui/button";
import { ArrowUp, Trash, X } from "lucide-react";
import {
  AlertDialog,
  AlertDialogAction,
  AlertDialogCancel,
  AlertDialogContent,
  AlertDialogDescription,
  AlertDialogFooter,
  AlertDialogHeader,
  AlertDialogTitle,
  AlertDialogTrigger,
} from "@/components/ui/alert-dialog";
import {
  Tooltip,
  TooltipContent,
  TooltipProvider,
  TooltipTrigger,
} from "@/components/ui/tooltip";
import { useState } from "react";
import { useChat } from "@/providers/ChatProvider";
import { ProgressSpinner } from "./ui/progress-spinner";

const ChatBox = () => {
  const { selectedCharacter } = useCharacter();
  const [input, setInput] = useState("");
  const { addUserMessage, inProgress } = useChat();

  const submit = (e: any) => {
    e.preventDefault();
    if (input.length > 0) {
      addUserMessage(input);
      setInput("");
    }
    if (input.length > 0) {
      const target = e.target as HTMLTextAreaElement;
      target.style.minHeight = "0px";
    }
  };

  return (
    <div className="bg-background fixed bottom-2 left-[16rem] w-[calc(100%_-_16.5rem)] px-4 pb-4 rounded-2xl">
      <span className="w-full left-0 -top-8 h-8 absolute bg-gradient-to-t from-background to-transparent"></span>
      {selectedCharacter && (
        <motion.form
          initial={{
            opacity: 0,
            scale: 0.9,
          }}
          animate={{
            opacity: 1,
            scale: 1,
          }}
          className="h-fit flex flex-col items-center space-y-2 p-2 rounded-xl border bg-card text-card-foreground shadow"
          onSubmit={(e) => submit(e)}
        >
          <textarea
            placeholder={`Message ${selectedCharacter?.alias}`}
            spellCheck={false}
            className="resize-none w-full flex-1 bg-transparent p-3 pb-1.5 text-sm outline-none min-h-0 ring-0 placeholder:text-gray-500 [&::-webkit-scrollbar]:hidden [-ms-overflow-style:none] [scrollbar-width:none] h-auto overflow-y-auto"
            onInput={(e) => {
              const target = e.target as HTMLTextAreaElement;
              target.style.minHeight = `${Math.min(
                target.scrollHeight,
                window.innerHeight - 10 * 16
              )}px`;
              setInput(target.value);
            }}
            onKeyDown={(e) => {
              if (e.key === "Enter" && !e.shiftKey && !inProgress) {
                submit(e);
              }
            }}
            value={input}
          ></textarea>
          <div className="flex w-full justify-between items-center">
            <ClearChatButton />
            <SubmitChatButton />
          </div>
        </motion.form>
      )}
    </div>
  );
};

const ClearChatButton = () => {
  const [dialogOpen, setDialogOpen] = useState(false);
  const { clearMessages } = useChat();

  return (
    <AlertDialog open={dialogOpen}>
      <AlertDialogTrigger asChild>
        <TooltipProvider>
          <Tooltip>
            <TooltipTrigger asChild onClick={() => setDialogOpen(true)}>
              <Button variant="ghost" className="!p-0 aspect-square">
                <Trash />
              </Button>
            </TooltipTrigger>
            <TooltipContent>
              <p>Clear Chat</p>
            </TooltipContent>
          </Tooltip>
        </TooltipProvider>
      </AlertDialogTrigger>
      <AlertDialogContent>
        <AlertDialogHeader>
          <AlertDialogTitle>Delete Chat History</AlertDialogTitle>
          <AlertDialogDescription>
            This action cannot be undone. This will all of your previous
            messages and responses to the agent from this session.
          </AlertDialogDescription>
        </AlertDialogHeader>
        <AlertDialogFooter>
          <AlertDialogCancel onClick={() => setDialogOpen(false)}>
            Cancel
          </AlertDialogCancel>
          <AlertDialogAction
            onClick={() => {
              setDialogOpen(false);
              clearMessages();
            }}
          >
            Delete Chat
          </AlertDialogAction>
        </AlertDialogFooter>
      </AlertDialogContent>
    </AlertDialog>
  );
};

const SubmitChatButton = () => {
  const { cancelLastMessage, inProgress } = useChat();

  return (
    <div className="flex space-x-2 items-center">
      {inProgress && (
        <TooltipProvider>
          <Tooltip>
            <TooltipTrigger asChild>
              <Button
                variant="ghost"
                className="!p-0 aspect-square"
                onClick={() => {
                  cancelLastMessage();
                }}
              >
                <X />
              </Button>
            </TooltipTrigger>
            <TooltipContent>
              <p>Cancel Request</p>
            </TooltipContent>
          </Tooltip>
        </TooltipProvider>
      )}
      <Button disabled={inProgress} type="submit">
        {inProgress ? <ProgressSpinner /> : <ArrowUp />}
      </Button>
    </div>
  );
};

export default ChatBox;
