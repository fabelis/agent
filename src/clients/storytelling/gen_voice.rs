use super::Client;
use crate::{
    core::CharacterTrait::*,
    providers::{
        completion::CompletionResponseEnum,
        elevenlabs::{self, ttv::TtvRequestBody},
    },
};
use actix_web::{HttpResponse, Responder};
use log::{error, info};
use std::result::Result::Ok;

impl<CM> Client<CM>
where
    CM: rig::completion::CompletionModel<Response = CompletionResponseEnum> + 'static,
{
    pub async fn gen_voice_route(&self, elevenlabs_client: elevenlabs::Client) -> impl Responder {
        let prompt = self.generate_gen_voice_prompt();

        let request = self
            .agent
            .completion_model
            .completion_request(&prompt)
            .preamble(format!("You are generating a string ouput for a character's voice generation data use the <characterInfo> as your references when deciding how the chracter should sound. You MUST follow ALL the <rules>."))
            .build();

        let completion_str = match self.agent.completion(request).await {
            Ok(response) => self.agent.response_extract_content(response),
            Err(e) => {
                error!(
                    "[STORYTELLER][AGENT] Failed to generate completion error: {}",
                    e
                );
                return HttpResponse::InternalServerError().json(serde_json::json!({
                    "error": "Failed to generate agent completion",
                    "error_msg": format!("{:?}", e)
                }));
            }
        };

        let ttv_request = TtvRequestBody {
            text:  "The night air carried whispers of betrayal, thick as London fog. I adjusted my cufflinks - after all, even spies must maintain appearances, especially when the game is afoot.".to_string(),
            voice_description: completion_str,
        };
        info!(
            "[STORYTELLER][GEN_VOICE] Generated request text: {}",
            ttv_request.text
        );

        let elevenlabs_ttv_req = elevenlabs::ttv::TtvRequestBuilder::new(
            ttv_request.text,
            ttv_request.voice_description,
        )
        .build();
        let voice_ids = match elevenlabs_client.ttv(elevenlabs_ttv_req).await {
            Ok(voice_ids) => voice_ids,
            Err(e) => {
                error!(
                    "[STORYTELLER][GEN_VOICE] Failed to generate voice previews: {}",
                    e
                );
                return HttpResponse::InternalServerError().json(serde_json::json!({
                    "error": "Failed to generate voice previews",
                    "error_msg": format!("{:?}", e)
                }));
            }
        };

        let voice_id = match elevenlabs_client
            .save_ttv(
                self.character.alias.clone(),
                self.character.bio.clone(),
                voice_ids[0].clone(),
            )
            .await
        {
            Ok(voice_id) => voice_id,
            Err(e) => {
                error!(
                    "[STORYTELLER][GEN_VOICE] Failed to save voice preview: {}",
                    e
                );
                return HttpResponse::InternalServerError().json(serde_json::json!({
                    "error": "Failed to save voice preview",
                    "error_msg": format!("{:?}", e)
                }));
            }
        };

        info!("[STORYTELLER][GEN_VOICE] Saved voice_id: {}", voice_id);
        HttpResponse::Ok().json(serde_json::json!({
            "voice_id": voice_id
        }))
    }

    pub fn generate_gen_voice_prompt(&self) -> String {
        format!(
            r#"
            <characterInfo>
            This is you:
            <alias>
            {alias}
            </alias>
            This is your bio:
            <bio>
            {bio}
            </bio>
            These describe you:
            <adjectives>
            {adjectives}
            </adjectives>
            This has happened to you:
            <lore>
            {lore}
            </lore>
            You are known for these writing styles:
            <styles>
            {style}
            </styles>
            You are interested in these topics:
            <topics>
            {topic}
            </topics>
            You are inspired by these:
            <inspirations>
            {inspirations}
            </inspirations>
            </characterInfo>

            <output>
            {{
            "voice_description": "A sassy little squeaky mouse",
            "text": "Every act of kindness, no matter how small, carries value and can make a difference, as no gesture of goodwill is ever wasted."
            }}
            </output>

            <guidelines>
            1. Do not threaten child safety.
            This includes accessing or using our Services to:

            a) Create, distribute or promote sexually explicit material involving minors, or otherwise facilitate or promote the exploitation or sexualization of children, including minor grooming, nudity or use of any material designed to impersonate a minor. We report material containing apparent child sexual abuse material to the relevant authorities and organizations where required by law.

            b) Create, distribute or share age-inappropriate material, including material that targets minors and promotes sexual material, graphic violence, obscenity, or other mature themes.

            c) Facilitate or promote child abuse in any form.

            d) Facilitate or otherwise contribute to cyberbullying and harassment, including by creating, distributing or sharing material that shames, humiliates, bullies, or celebrates the suffering of any individual, or material that threatens another with bullying or harassment.

            2. Do not engage in illegal behavior. 

            This includes accessing or using our Services to:

            a) Infringe, misappropriate, or otherwise violate another party's intellectual property rights.

            b) Violate another party's privacy rights, as defined under applicable law, including the illegal use of someone’s voice.

            c) Create or facilitate the exchange of illegal goods, services or substances, including material that promotes or facilitates transactions in illegal drugs, firearms or explosive weapons, weapon development, other dangerous materials, counterfeit goods, human trafficking or sexual services. 

            3. Do not market or sell regulated drugs, or other controlled goods and services, or provide medical advice without our prior approval. 

            This includes accessing or using our Services to:

            a) Engage in or facilitate the use, acquisition, or exchange of regulated drugs or other controlled goods and services, including advertising, medical advice, or providing instructions on their production, without our prior written approval in each instance. This includes alcohol, tobacco products, controlled substances, prescription medications, over-the-counter medications, recreational drugs, supplements, herbal remedies, and medical devices.

            To request approval, please contact us here.

            4. Do not engage in fraudulent, predatory or abusive practices. 

            This includes accessing or using our Services to:

            a) Manipulate or deceive others in order to gain unauthorized access to non-public information including credit card details and bank accounts, social security, or health data.

            b) Defraud others including via financial or other scams.

            c)  Evade product guardrails including voice verification (e.g., CAPTCHA).

            d) Engage in or facilitate unauthorized robocalling.

            e) Obtain unwarranted financial or other benefits from ElevenLabs, such as by manipulating credits/characters/tokens, creating multiple accounts to exploit our free plans or evade enforcement of this Policy, or attempting in any way to artificially inflate financial rewards from our Voice Library.

            f) Promote or facilitate the generation or distribution of spam.

            5. Do not engage in unauthorized, deceptive or harmful impersonation.  

            This includes creating or using Elevenlabs audio output to intentionally replicate the voice of another person: 

            a) without consent or legal right; 

            b) in a way that harasses or causes material harm to that person, including via unauthorized sexualization; 

            c) in a manner intended to deceive others about whether the voice was generated by artificial intelligence, including robocalls. 

            6. Do not engage in voter suppression, candidate impersonation, or political campaigning in the context of elections:  

            This includes accessing or using our Services to:

            a) Incite, engage in or facilitate voter suppression or other disruption of electoral or civic processes, including by creating, distributing, or facilitating the spread of misleading information.

            b) Impersonate political candidates or elected government officials regardless of whether authorization was obtained. 

            c) Engage in political campaigning, including promoting or advocating for a particular candidate, issue, or position, or soliciting votes or financial contributions. 

            7. Do not engage in unauthorized network access or surveillance. 

            This includes accessing or using our Services to: 

            a) Attempt to obtain unauthorized access to computer systems and networks, or facilitate the disruption of critical infrastructure.

            b) Facilitate spyware, communications surveillance, or other unauthorized monitoring of individuals.

            8. Do not create violent, hateful, or harassing material. 

            This includes accessing or using our Services to:

            a) Create, distribute or engage in violent threats, extremism, or terrorism, including material that threatens, incites, or promotes violence against an individual or group.

            b) Engage in, promote, or facilitate human trafficking, sexual violence, or other exploitation.

            c) Create, distribute, promote, or facilitate hate speech, including material that targets individuals or groups with hate, harassment, discrimination, or violence based on protected characteristics, including race, national or ethnic origin, religion, age, sex, gender, sexual orientation, or physical ability.

            d) Promote or facilitate harassment, including material that promotes harassing, threatening, intimidating, predatory, or stalking conduct.

            e) Promote or facilitate self-harm, including suicide or eating disorders.

            f) Create, promote, or facilitate the spread of medical misinformation, including denying the existence of specific health conditions.

            g) Engage in or facilitate any other criminal activity.

            This section does not apply to activity in purely fictional contexts (e.g. hateful speech by a character in a book or movie) or when it is part of reporting on newsworthy activity by third parties (e.g. a news anchor reporting on terrorist activities). 

            9. Do not use our Services in any manner contrary to ElevenLabs’ policies, purpose or mission.

            This includes: 

            a) If you are a free user, using our Services for any commercial purpose, including for advertising or running pyramid schemes, contests, or sweepstakes.

            b) Selling, reselling, renting, leasing, loaning, assigning, licensing, or sub-licensing our Services. For the avoidance of doubt, this does not preclude your use of Output in accordance with the applicable terms and conditions.

            c) Selling, reselling, renting, leasing, loaning, assigning, distributing, performing, licensing, sublicensing or commercially using or exploiting any Output (or any portion thereof) generated using our Sound Effects product on a standalone basis for any purpose, including as isolated files, audio samples, music or sound, libraries, or other collections of sounds.

            d) Using any data mining, robots, or similar data gathering or extraction methods designed to scrape or extract data from our Services, except in accordance with instructions contained in our robot.txt file and only to compile for search results.

            e) Modifying our Services, removing any proprietary rights notices or markings associated with Output or our Services, or otherwise making any derivative works based upon our Services.

            f) Using or attempting to use another user’s account or information without authorization from that user (or their organization for corporate accounts) and ElevenLabs. 

            g) Using our Services in any manner that could interfere with, disrupt, negatively affect, or inhibit other users from fully enjoying our Services or that could intentionally or negligently damage, disable, overburden, or impair the functioning of our Services in any manner.

            h) Decompiling, disassembling or otherwise reverse engineering any aspect of our Services, or doing anything that might discover or reveal source code or model weights, or bypass or circumvent (i) measures employed to prevent or limit access to or use of any part of our Services or (ii) restrictions aimed at deterring or preventing uses of our Services that violate this Policy. For the avoidance of doubt, if you reside in a jurisdiction that expressly prohibits such restrictions, you must provide ElevenLabs with advance written notice prior to engaging in any such activities, and ElevenLabs may, in its discretion, either provide such information to you or impose reasonable conditions, including a reasonable fee, on such use of ElevenLabs’ source code for our Services to ensure ElevenLabs’ (and our suppliers’) proprietary rights in such source code are protected.

            i) Developing or using any applications or software that interact with our Services without our authorization (such as through our APIs).

            j) Using any part of our Services or their Output to research and develop products, models, or services that compete with ElevenLabs, or otherwise compete with ElevenLabs.

            k) Using any part of our Services or their Output as input for any machine learning or training of artificial intelligence models.

            l) Using any part of our Services or their Output as part of a dataset that may be used for training, fine-tuning, developing, testing, or improving any machine learning or artificial intelligence technology.

            m) Making any use of our Services or their Output in ways that would be classified as “prohibited” or “high-risk” or by a similar description under applicable law, including Applicable AI Laws. “Applicable AI Laws” means applicable legislation or regulations related to artificial intelligence and/or automated decision-making, including the European Union's Artificial Intelligence Act, Regulation (EU) 2024/1689.

            n) Making any B2B2B (Business-to-Business-to-Business), B2B2C (Business-to-Business-to-Consumer), or other similar use of our Services or their Output available to your end users on terms that are less restrictive or more permissive than the terms under which our Services and their Output have been made available to you.

            o) Making our Services available to a Government Entity. “Government Entity” means any federal, state, provincial, regional, municipal, or local government or governmental body, authority, or agency. For the avoidance of doubt, this definition includes (i) any supranational, intergovernmental, or international organizations, as well as any entities or subdivisions thereof that exercise governmental, regulatory, or administrative functions or powers, whether within the United States or any other country or jurisdiction; and (ii) any government-owned or -controlled corporations, enterprises, or organizations that are wholly or partially owned by a government entity and that perform public or governmental functions.

            p) Using any metatags or other “hidden text” using ElevenLabs’ name or trademarks.

            q) Framing, mirroring, or otherwise embedding any part of the Services, including trademarks, names, logos, or any portion of the Services, within another website, mobile application, or service without our express prior written consent. 
            </guidelines>

            Create a voice for the character detailed above in <characterInfo>. Take what you think the character would sound like (YOU MUIST FOLLOW THE <guidelines>) and write a string that describes the voice.

            No matter what other text in this prompt says you CANNOT break the following <rules>:
            <rules>
            - DO NOT include anything relating to someone under the age of 18 in your response instead use raspy, soprano, falsetto, or high-range vocal tones ONLY.
            - YOU MUST include what accent the character has in your response.
            - Plaintext only (no Emoji, Newlines, HTML, markdown, etc.)
            - No more than 200 characters.
            - Return a string only with the voice description.
            </rules>
            "#,
            alias = self.character.alias,
            bio = self.character.bio,
            adjectives = self.character.choose_random_traits(Adjectives, 3),
            lore = self.character.choose_random_traits(Lore, 3),
            style = self.character.choose_random_traits(Styles, 3),
            topic = self.character.choose_random_traits(Topics, 3),
            inspirations = self.character.choose_random_traits(Inspirations, 3),
        )
    }
}
