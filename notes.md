### anchor-spl
- TokenAccount is a PDA owned by the token_prgram contains necessary information about the token like mint_program,owner etc.
- TokenInterface contains programID's of anchor-spl programs legacy and 2022
- Interface is used for program validation no data handling
- InterfaceAccount is used for both the passed program validation and deserializes the passed data @reminder - read token-2022 programs
- mint::token_program = token_Prgram validates the mint account belongs to the token_program in the context