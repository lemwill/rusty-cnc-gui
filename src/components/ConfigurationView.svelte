<script>
  import protobuf from "protobufjs";
  import { onMount } from "svelte";
  import { writable } from "svelte/store";
  import { Accordion, AccordionItem } from "@skeletonlabs/skeleton";
  import { LightSwitch } from "@skeletonlabs/skeleton";
  import { SlideToggle } from "@skeletonlabs/skeleton";
  import {
    socket,
    MessageFromInterface,
    JogMessage,
  } from "../lib/socketStore.js";

  const PROTO_PATH = "src/protobuf/jog_message.proto";
  const PROTO_TYPE = "messages_proto.Configuration";

  let selectedSection = null;
  const selectedField = writable(null);
  const sections = writable([]);

  const loadProto = async () => {
    const root = await protobuf.load(PROTO_PATH);
    const Configuration = root.lookupType(PROTO_TYPE);
    Configuration.resolve();
    return Configuration;
  };

  const loadMessage = async (Configuration) => {
    const message = Configuration.create();

    message.general = { junctionDeviation: 5 };
    message.limits = {
      invertLimitX: true,
      invertLimitY: false,
      invertLimitZ: true,
    };
    message.feedrate = {
      rapidXFeedrate: 2,
      rapidYFeedrate: 3,
      rapidZFeedrate: 1,
    };

    return message;
  };

  const camelCaseToCapitalizedWords = (str) =>
    str
      .replace(/(?:^|([A-Z]))/g, (_, y) => " " + (y ? y : ""))
      .trim()
      .replace(/\b\w/g, (l) => l.toUpperCase());

  const processFields = (fieldArray) =>
    fieldArray.map((field) => {
      const options = field.options || {};
      return {
        name: field.name,
        description: options["(description)"] || "",
        min: options["(min)"] || 0,
        max: options["(max)"] || Infinity,
        value: 1,
        type: field.type,
      };
    });

  onMount(async () => {
    const Configuration = await loadProto();

    if (Configuration) {
      Configuration.resolveAll();

      const tempSections = Configuration.fieldsArray.map((field) => {
        const messageType = field.resolvedType;
        if (!messageType) {
          return null;
        }
        return {
          name: field.name,
          fields: processFields(messageType.fieldsArray),
        };
      });

      const initialMessage = await loadMessage(Configuration);

      tempSections.forEach((section) => {
        if (section) {
          section.fields.forEach((field) => {
            field.value = initialMessage[section.name][field.name];
          });
        }
      });

      sections.set(tempSections);
    }
  });

  const handleSubmit = async () => {
    const configuration = {};
    const Configuration = await loadProto();

    $sections.forEach((section) => {
      if (!configuration[section.name]) {
        configuration[section.name] = {};
      }
      section.fields.forEach((field) => {
        configuration[section.name][field.name] = field.value;
      });
    });
    console.log("Configuration", configuration);

    const errMsg = Configuration.verify(configuration);
    if (errMsg) throw Error(errMsg);

    // Create a MessageFromInterface
    let sendMsg = MessageFromInterface.create({
      // ... include other fields if necessary
      configuration: Configuration.create(configuration),
    });

    const errMsg2 = MessageFromInterface.verify(sendMsg);
    if (errMsg2) throw Error(errMsg2);

    // Encode the MessageFromInterface
    let buffer = MessageFromInterface.encode(sendMsg).finish();
    console.log("Sending message", sendMsg);
    // Send the buffer via the WebSocket
    socket.send(buffer);
  };

  const selectField = (section, field) => {
    selectedSection = section;
    selectedField.set(field);
  };
</script>

<div class="treeview">
  <Accordion autocollapse>
    {#each $sections as section (section.name)}
      <AccordionItem open>
        <svelte:fragment slot="summary">{section.name}</svelte:fragment>
        <svelte:fragment slot="content">
          <ul>
            {#each section.fields as field (field.name)}
              <li>
                <a
                  href="#"
                  on:click|preventDefault={() => selectField(section, field)}
                >
                  {field.name}
                </a>
              </li>
            {/each}
          </ul>
        </svelte:fragment>
      </AccordionItem>
    {/each}

    <!-- ... -->
  </Accordion>
  {#if $selectedField}
    <div class="editor">
      <h2>{camelCaseToCapitalizedWords($selectedField.name)}</h2>
      <form on:submit|preventDefault={handleSubmit}>
        <label for={$selectedField.name}>{$selectedField.description}</label>
        {#if $selectedField.type === "int32"}
          <input
            type="number"
            class="input"
            title="Input (number)"
            id={$selectedField.name}
            min={$selectedField.min}
            max={$selectedField.max}
            bind:value={$selectedField.value}
            step="1"
          />
        {:else if $selectedField.type === "double" || $selectedField.type === "float"}
          <input
            type="number"
            class="input"
            title="Input (number)"
            id={$selectedField.name}
            min={$selectedField.min}
            max={$selectedField.max}
            bind:value={$selectedField.value}
            step="0.1"
          />
        {:else if $selectedField.type === "bool"}
          <SlideToggle
            name={$selectedField.name}
            bind:checked={$selectedField.value}
          />
        {/if}
        <br />
        <button type="submit" class="btn variant-filled">Submit</button>
      </form>
    </div>
  {/if}
</div>

<style>
  .container {
    display: flex;
  }

  .treeview {
    display: flex;

    flex: 1 1 auto; /* Don't grow or shrink */
    width: 100%; /* You can set this to whatever you want */
    margin-right: 5%; /* Add a margin to the right */
  }

  .editor {
    display: flex;
    flex-direction: column;
    flex: 1 1 auto; /* Grow and shrink as needed */
    width: 100%; /* You can set this to whatever you want */
    margin-right: 5%;
  }
</style>
