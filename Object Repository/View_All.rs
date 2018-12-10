<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>View_All</name>
   <tag></tag>
   <elementGuidId>44ad229d-09cb-4c1c-a4ab-a2a21d031ba2</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\&quot;collection_name\&quot;:\&quot;rcmb_systems\&quot;,\&quot;search\&quot;:{\&quot;find\&quot;:\&quot;\&quot;,\&quot;columns\&quot;:[\&quot;system_name\&quot;,\&quot;system_category\&quot;,\&quot;app_type\&quot;,\&quot;defaultBrowser\&quot;,\&quot;system_url\&quot;,\&quot;concurrent_users\&quot;]},\&quot;limit\&quot;:25,\&quot;skip\&quot;:0,\&quot;sort_key\&quot;:\&quot;system_name\&quot;,\&quot;sort_order\&quot;:1,\&quot;filters\&quot;:{}}&quot;,
  &quot;contentType&quot;: &quot;application/json&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>eyJhbGciOiJSUzI1NiIsInR5cCIgOiAiSldUIiwia2lkIiA6ICIwZ2hQTS1feVNFRTMxdVoxVUxNSW1QSDBjbDlkbVR1NXUzdDRCV3ZmV3RjIn0.eyJqdGkiOiI0N2U5MWIwOS05YTRjLTQzODAtODNjNi05ZTcxNDQ1MTI2YWEiLCJleHAiOjE1NDUwMTk1MjYsIm5iZiI6MCwiaWF0IjoxNTQ0MTU1NTI4LCJpc3MiOiJodHRwczovL2lhbWF1dGgucmNtYnJhaW4uY29tL2F1dGgvcmVhbG1zL3JjbWJfcmVhbG1fc3RhZ2luZyIsImF1ZCI6InJjbWJfY2xpZW50X3N0YWdpbmciLCJzdWIiOiI3ZGNjZGI1Zi0xODE5LTRjMDItYTJiMi1iZjUxYzM4ZWY2OGMiLCJ0eXAiOiJCZWFyZXIiLCJhenAiOiJyY21iX2NsaWVudF9zdGFnaW5nIiwibm9uY2UiOiJkZjVkMDQyNy1mZTdhLTQ3MWEtODMyMi04MjBjYzhkYTY1ZTIiLCJhdXRoX3RpbWUiOjE1NDQxNTU1MjYsInNlc3Npb25fc3RhdGUiOiJjY2NhMGYwZC0xZTRhLTQ5NGQtOGMyMS02YjU3MDYwMDE0OWUiLCJhY3IiOiIxIiwiYWxsb3dlZC1vcmlnaW5zIjpbIioiXSwicmVzb3VyY2VfYWNjZXNzIjp7ImFjY291bnQiOnsicm9sZXMiOlsibWFuYWdlLWFjY291bnQiLCJtYW5hZ2UtYWNjb3VudC1saW5rcyIsInZpZXctcHJvZmlsZSJdfX0sIm5hbWUiOiJJbnRlcm5hbCBBZG1pbiIsInByZWZlcnJlZF91c2VybmFtZSI6ImludGVybmFsYWRtaW5AaWluZXJkcy5jb20iLCJnaXZlbl9uYW1lIjoiSW50ZXJuYWwgQWRtaW4iLCJmYW1pbHlfbmFtZSI6IiIsImVtYWlsIjoiaW50ZXJuYWxhZG1pbkBpaW5lcmRzLmNvbSIsImNsaWVudFJvbGVzIjoiW3tcImNsaWVudF9pZFwiOlwiXCIsXCJjbGllbnRfbmFtZVwiOlwiXCIsXCJyb2xlX2lkXCI6XCJSMDA1XCIsXCJyb2xlX25hbWVcIjpcIlJDTSBEYXRhIEFkbWluXCJ9LHtcIldGX1JPTEVfSURcIjpcIlIwMDFcIn1dIn0.TzNVrU9ntsUkaI79yXaTohSP9NGdLGS_ou1r6fe4BnfCGu8GkX6BZrEtnPf7fkD9kldSCmarbhRK4VoTBREZY1EEVY0nRakMCF-AI7qZLCVNZ8x6wsRcyhqMBx2hokK_is6vbkKZThlMfFeheD6iiuO_NvytMse5-t7uRc7PAsFD_v_TW5DQjUPULaARUVNs7yufFZdkwLkKzPyqOk-qzcftvTCL95TCJWq1eefoJ-LAGXn7_ffU7bSeAeD2OgiGl9sG3aVQCnsTf5k7leYbmClRk99Rb3ZF7nLcZfPxjJMZ6HlFQtZ3Fwk2D4iO8pJVlsrecaWz130yHAH80JqcyA</value>
   </httpHeaderProperties>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>https://rcmbstaging-datamgmt.iinerds.com/find_collection_records/</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceFunction></soapServiceFunction>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()



WS.verifyResponseStatusCode(response, 200)

assertThat(response.getStatusCode()).isEqualTo(200)

WS.verifyElementPropertyValue(response, 'data[1].defaultBrowser', 'chrome')</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
