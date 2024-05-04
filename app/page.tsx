import Image from "next/image";
import Greet from "@/app/greet";
import GuidGenerator from "@/app/guid_generator";
import RequestPage from "@/app/request_page";

export default function Home() {
  return (
    <main className="">
      <RequestPage />
    </main>
  );
}
