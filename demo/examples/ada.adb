with Ada.Text_IO; use Ada.Text_IO;

procedure Hello is
   type Day is (Mon, Tue, Wed, Thu, Fri, Sat, Sun);
   Today : Day := Wed;
   Count : Integer := 42;
begin
   Put_Line("Hello from Ada!");
   if Today = Wed then
      Put_Line("It's Wednesday");
   end if;
end Hello;
