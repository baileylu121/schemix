
%option noyywrap

%%

[a\]z]	{ printf("matched %s\n",yytext); }
(.|\n)	{}

%%

int main()
{
  yylex();

  return 0;
}
